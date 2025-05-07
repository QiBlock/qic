use std::{env, path::PathBuf};

mod codegen;
mod flags;

fn main() -> anyhow::Result<()> {
    let flags = flags::Xtask::from_env_or_exit();
    let sh = xshell::Shell::new()?;
    sh.change_dir(project_root());

    match flags.subcommand {
        flags::XtaskCmd::Codegen(cmd) => cmd.run(&sh),
    }
}

fn project_root() -> PathBuf {
    let dir =
        env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned());
    PathBuf::from(dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned()
}
