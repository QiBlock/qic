use qi_compiler::yul::lexer::Lexer;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let dir =
        env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned());
    let source = fs::read_to_string(
        PathBuf::from(dir)
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("doc/ERC20.sol.ERC20.yul"),
    )
    .unwrap();

    let mut lexer = Lexer::new(source.as_str());
    while let Some(token) = lexer.next() {
        if token.kind.is_err() {
            panic!("Error tokenizing: {:?}", token);
        } else if token.kind.unwrap().is_trivia() {
            continue;
        }
        println!("{:?}", token);
    }
}
