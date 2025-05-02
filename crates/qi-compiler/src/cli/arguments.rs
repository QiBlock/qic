use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;

use clap::Parser;
use path_slash::PathExt;

#[derive(Debug, Parser)]
#[command(name = "The Qi compiler", arg_required_else_help = true)]
pub struct Arguments {
    /// Print the version and exit.
    #[arg(long = "version")]
    pub version: bool,

    /// Specify the input paths and remappings.
    /// If an argument contains a '=', it is considered a remapping.
    /// Multiple Solidity files can be passed in the default Solidity mode.
    /// Yul, LLVM IR, and PolkaVM Assembly modes currently support only a single file.
    pub inputs: Vec<String>,

    /// Set the given path as the root of the source tree instead of the root of the filesystem.
    /// Passed to `solc` without changes.
    #[arg(long = "base-path")]
    pub base_path: Option<String>,

    /// Make an additional source directory available to the default import callback.
    /// Can be used multiple times. Can only be used if the base path has a non-empty value.
    /// Passed to `solc` without changes.
    #[arg(long = "include-path")]
    pub include_paths: Vec<String>,

    /// Allow a given path for imports. A list of paths can be supplied by separating them with a comma.
    /// Passed to `solc` without changes.
    #[arg(long = "allow-paths")]
    pub allow_paths: Option<String>,

    /// Create one file per component and contract/file at the specified directory, if given.
    #[arg(short = 'o', long = "output-dir")]
    pub output_directory: Option<PathBuf>,

    /// Specify the path to the `solc` executable. By default, the one in `${PATH}` is used.
    /// Yul mode: `solc` is used for source code validation, as `resolc` itself assumes that the input Yul is valid.
    /// LLVM IR mode: `solc` is unused.
    #[arg(long = "solc")]
    pub solc: Option<String>,

    /// Set the optimization parameter -O[0 | 1 | 2 | 3 | s | z].
    /// Use `3` for best performance and `z` for minimal size.
    #[arg(short = 'O', long = "optimization")]
    pub optimization: Option<char>,

    /// Try to recompile with -Oz if the bytecode is too large.
    #[arg(long = "fallback-Oz")]
    pub fallback_to_optimizing_for_size: bool,

    /// Disable the `solc` optimizer.
    /// Use it if your project uses the `MSIZE` instruction, or in other cases.
    /// Beware that it will prevent libraries from being inlined.
    #[arg(long = "disable-solc-optimizer")]
    pub disable_solc_optimizer: bool,

    /// The EVM target version to generate IR for.
    /// See https://github.com/paritytech/revive/blob/main/crates/common/src/evm_version.rs for reference.
    #[arg(long = "evm-version")]
    pub evm_version: Option<String>,

    /// Specify addresses of deployable libraries. Syntax: `<libraryName>=<address> [, or whitespace] ...`.
    /// Addresses are interpreted as hexadecimal strings prefixed with `0x`.
    #[arg(short = 'l', long = "libraries")]
    pub libraries: Vec<String>,

    /// These are passed to LLVM as the command line to allow manual control.
    #[arg(long = "llvm-arg")]
    pub llvm_arguments: Vec<String>,

    /// Set the verify-each option in LLVM.
    /// Only for testing and debugging.
    #[arg(long = "llvm-verify-each")]
    pub llvm_verify_each: bool,

    /// Set the debug-logging option in LLVM.
    /// Only for testing and debugging.
    #[arg(long = "llvm-debug-logging")]
    pub llvm_debug_logging: bool,

    /// Suppress specified warnings.
    /// Available arguments: `ecrecover`, `sendtransfer`, `extcodesize`, `txorigin`, `blocktimestamp`, `blocknumber`, `blockhash`.
    #[arg(long = "suppress-warnings")]
    pub suppress_warnings: Option<Vec<String>>,

    /// Generate source based debug information in the output code file. This only has an effect
    /// with the LLVM-IR code generator and is ignored otherwise.
    #[arg(short = 'g')]
    pub emit_source_debug_info: bool,

    /// Dump all IRs to files in the specified directory.
    /// Only for testing and debugging.
    #[arg(long = "debug-output-dir")]
    pub debug_output_directory: Option<PathBuf>,
}

impl Arguments {
    /// Validate the arguments.
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.version && std::env::args().count() > 2 {
            anyhow::bail!("No other options are allowed while getting the compiler version.");
        }

        Ok(())
    }

    /// Returns remappings from input paths.
    pub fn split_input_files_and_remappings(
        &self,
    ) -> anyhow::Result<(Vec<PathBuf>, Option<BTreeSet<String>>)> {
        let mut input_files = Vec::with_capacity(self.inputs.len());
        let mut remappings = BTreeSet::new();

        for input in self.inputs.iter() {
            if input.contains('=') {
                let mut parts = Vec::with_capacity(2);
                for path in input.trim().split('=') {
                    let path = PathBuf::from(path);
                    parts.push(
                        Self::path_to_posix(path.as_path())?
                            .to_string_lossy()
                            .to_string(),
                    );
                }
                if parts.len() != 2 {
                    anyhow::bail!(
                        "Invalid remapping `{}`: expected two parts separated by '='",
                        input
                    );
                }
                remappings.insert(parts.join("="));
            } else {
                let path = PathBuf::from(input.trim());
                let path = Self::path_to_posix(path.as_path())?;
                input_files.push(path);
            }
        }

        let remappings = if remappings.is_empty() {
            None
        } else {
            Some(remappings)
        };

        Ok((input_files, remappings))
    }

    /// Normalizes an input path by converting it to POSIX format.
    fn path_to_posix(path: &Path) -> anyhow::Result<PathBuf> {
        let path = path
            .to_slash()
            .ok_or_else(|| anyhow::anyhow!("Input path {:?} POSIX conversion error", path))?
            .to_string();
        let path = PathBuf::from(path.as_str());
        Ok(path)
    }
}
