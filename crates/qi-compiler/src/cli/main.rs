pub mod arguments;

use arguments::Arguments;
use qi_compiler::{Compiler, DebugConfig, OptimizerSettings, SolcCompiler, Version};
use revive_solc_json_interface::{
    SolcStandardJsonInput, SolcStandardJsonInputLanguage, SolcStandardJsonInputSettingsOptimizer,
    SolcStandardJsonInputSettingsSelection,
};
use std::{collections::BTreeMap, io::Write};

/// The common application success exit code.
pub const EXIT_CODE_SUCCESS: i32 = 0;

/// The common application failure exit code.
pub const EXIT_CODE_FAILURE: i32 = 1;

fn main() -> anyhow::Result<()> {
    std::process::exit(match main_inner() {
        Ok(()) => EXIT_CODE_SUCCESS,
        Err(error) => {
            writeln!(std::io::stderr(), "{error}")?;
            EXIT_CODE_FAILURE
        }
    })
}

fn main_inner() -> anyhow::Result<()> {
    let arguments = <Arguments as clap::Parser>::try_parse()?;
    arguments.validate()?;

    if arguments.version {
        writeln!(
            std::io::stdout(),
            "{} version {}",
            env!("CARGO_PKG_DESCRIPTION"),
            Version::default().long
        )?;
        return Ok(());
    }

    let debug_config = match arguments.debug_output_directory {
        Some(ref debug_output_directory) => {
            std::fs::create_dir_all(debug_output_directory.as_path())?;
            DebugConfig::new(
                Some(debug_output_directory.to_owned()),
                arguments.emit_source_debug_info,
            )
        }
        None => DebugConfig::new(None, arguments.emit_source_debug_info),
    };

    let (input_files, remappings) = arguments.split_input_files_and_remappings()?;

    let suppressed_warnings = match arguments.suppress_warnings {
        Some(warnings) => Some(revive_solc_json_interface::ResolcWarning::try_from_strings(
            warnings.as_slice(),
        )?),
        None => None,
    };

    let mut solc = {
        #[cfg(not(target_os = "emscripten"))]
        {
            SolcCompiler::new(
                arguments
                    .solc
                    .unwrap_or_else(|| SolcCompiler::DEFAULT_EXECUTABLE_NAME.to_owned()),
            )?
        }
    };

    let solc_version = solc.version()?;

    let mut optimizer_settings = match arguments.optimization {
        Some(mode) => OptimizerSettings::try_from_cli(mode)?,
        None => OptimizerSettings::cycles(),
    };
    if arguments.fallback_to_optimizing_for_size {
        optimizer_settings.enable_fallback_to_size();
    }
    optimizer_settings.is_verify_each_enabled = arguments.llvm_verify_each;
    optimizer_settings.is_debug_logging_enabled = arguments.llvm_debug_logging;

    let solc_input = SolcStandardJsonInput::try_from_paths(
        SolcStandardJsonInputLanguage::Solidity,
        None,
        input_files.as_slice(),
        arguments.libraries,
        remappings,
        SolcStandardJsonInputSettingsSelection::new_required(),
        SolcStandardJsonInputSettingsOptimizer::new(
            !arguments.disable_solc_optimizer,
            None,
            &solc_version.default,
            optimizer_settings.is_fallback_to_size_enabled(),
        ),
        None,
        suppressed_warnings,
    )?;

    let solc_output = solc.standard_json(
        solc_input,
        arguments.base_path,
        arguments.include_paths,
        arguments.allow_paths,
    )?;

    if let Some(errors) = solc_output.errors.as_deref() {
        let mut has_errors = false;

        for error in errors.iter() {
            if error.severity.as_str() == "error" {
                has_errors = true;
            }

            writeln!(std::io::stderr(), "{error}")?;
        }

        if has_errors {
            anyhow::bail!("Error(s) found. Compilation aborted");
        }
    }

    let files = match solc_output.contracts.as_ref() {
        Some(files) => files,
        None => match &solc_output.errors {
            Some(errors) if errors.iter().any(|e| e.severity == "error") => {
                anyhow::bail!(serde_json::to_string_pretty(errors).expect("Always valid"));
            }
            _ => &BTreeMap::new(),
        },
    };
    // let mut project_contracts = BTreeMap::new();

    for (path, contracts) in files.iter() {
        for (name, contract) in contracts.iter() {
            let full_path = format!("{path}:{name}");

            let ir_optimized = match contract.ir_optimized.to_owned() {
                Some(ir_optimized) => ir_optimized,
                None => continue,
            };
            if ir_optimized.is_empty() {
                continue;
            }

            debug_config.dump_yul(full_path.as_str(), ir_optimized.as_str())?;
        }
    }

    Ok(())
}
