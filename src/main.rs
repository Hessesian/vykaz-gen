use vykazy::{InputType, parse_text};
use std::env;
use std::path::PathBuf;

fn main() -> Result<(), String> {
    let path = PathBuf::from(env::args_os().nth(1).expect("File path"));

    let res = parse_text(InputType::PATH(path))?;
    print!("{}", res);
    Ok(())
}
