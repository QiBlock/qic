pub mod arguments;
use arguments::Arguments;
use std::io::Write;

fn main() -> anyhow::Result<()> {
    std::process::exit(match main_inner() {
        Ok(()) => qi_compiler::EXIT_CODE_SUCCESS,
        Err(error) => {
            writeln!(std::io::stderr(), "{error}")?;
            qi_compiler::EXIT_CODE_FAILURE
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
            qi_compiler::versions::Version::default().long
        )?;
        return Ok(());
    }
    Ok(())
}
