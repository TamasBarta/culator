pub mod evaluator;
pub mod lexer;
pub mod parser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_expressions() {
        let test_cases = [
            ("2 * 3 / 4 ^ 2", 0.375),
            ("2 + 3 * 4", 14.0),
            ("10 - 2 * 3", 4.0),
            ("(2 + 3) * 4", 20.0),
            ("2 ^ 3 ^ 2", 512.0),
            ("8 / 2 / 2", 2.0),
            ("-5 + 3 * 2", 1.0),
            // ("2 * -3", -6.0),
            ("2 * (23 - 2.5 *2) ^2 /2 *3 + log(231)", 977.4424177105218),
            ("abs(-3) + 4 * 2", 11.0),
        ];

        for (expression, expected) in test_cases.iter() {
            let tokens = lexer::lex(*expression).expect("Lexing failed");
            let parsed = parser::parse(tokens.as_slice()).expect("Parse failed");
            let result = evaluator::evaluate(&parsed).expect("Evaluation failed");
            let diff = (result - expected).abs();
            assert!(
                diff < 1e-6,
                "{} => {} (expected {})",
                expression,
                result,
                expected
            );
        }
    }
}
