#![feature(iter_advance_by)]

use crate::{lexer::lex, parser::parse};

mod lexer;
mod parser;

pub fn main() -> Result<(), String> {
    println!("Hello, world!");
    let result = parse(lex("((2))").map_err(|_| "Not yet")?.as_slice())?;

    println!("{:?}", result);

    Ok(())
}
