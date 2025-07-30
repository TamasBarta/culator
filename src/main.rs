#![feature(iter_advance_by)]

use crate::{evaluator::evaluate, lexer::lex, parser::parse};

mod evaluator;
mod lexer;
mod parser;

pub fn main() -> Result<(), String> {
    let test_cases = [
        "2 * 3 / 4 ^ 2", // Expected: 0.375 (2 * (3 / (4 ^ 2)))
        "2 + 3 * 4",     // Expected: 14 (2 + (3 * 4))
        "10 - 2 * 3",    // Expected: 4 (10 - (2 * 3))
        "(2 + 3) * 4",   // Expected: 20 ((2 + 3) * 4)
        "2 ^ 3 ^ 2",     // Expected: 512 (2 ^ (3 ^ 2))
        "8 / 2 / 2",     // Expected: 2 ((8 / 2) / 2)
        "-5 + 3 * 2",    // Expected: 1 (-5 + (3 * 2))
        "2 * -3",        // Expected: -6 (2 * (-3))
        "2 * (23 - 2.5 *2) ^2 /2 *3 + log(231)",
        "abs(-3) + 4 * 2", // Expected: 11 (abs(-3) + (4 * 2))",
    ];

    println!("Testing complex expressions with new operators:");

    for test_input in test_cases {
        println!("\nExpression: {}", test_input);

        let tokens = lex(test_input).map_err(|_| "Lexing failed")?;
        println!("Tokens: {:?}", tokens);

        match parse(tokens.as_slice()) {
            Ok(expression) => {
                println!("Parsed: {:?}", expression);
                match evaluate(&expression) {
                    Ok(result) => println!("Result: {}", result),
                    Err(e) => println!("Evaluation error: {}", e),
                }
            }
            Err(e) => println!("Parse error: {:?}", e),
        }
    }

    Ok(())
}
