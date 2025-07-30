use crate::parser::Expression;

/// Evaluates a parsed expression and returns the numeric result
pub fn evaluate(expression: &Expression) -> Result<f64, &'static str> {
    match expression {
        Expression::NumericLiteral(value) => Ok(*value),

        Expression::Addition(left, right) => {
            let left_val = evaluate(left)?;
            let right_val = evaluate(right)?;
            Ok(left_val + right_val)
        }

        Expression::Subtraction(left, right) => {
            let left_val = evaluate(left)?;
            let right_val = evaluate(right)?;
            Ok(left_val - right_val)
        }

        Expression::Minus(operand) => {
            let val = evaluate(operand)?;
            Ok(-val)
        }

        Expression::Multiplication(left, right) => {
            let left_val = evaluate(left)?;
            let right_val = evaluate(right)?;
            Ok(left_val * right_val)
        }

        Expression::Division(left, right) => {
            let left_val = evaluate(left)?;
            let right_val = evaluate(right)?;
            if right_val == 0.0 {
                Err("Division by zero")
            } else {
                Ok(left_val / right_val)
            }
        }

        Expression::Exponentiation(left, right) => {
            let left_val = evaluate(left)?;
            let right_val = evaluate(right)?;
            Ok(left_val.powf(right_val))
        }

        Expression::FunctionCall(name, args) => match (name.as_str(), args.len()) {
            ("log", 1) => {
                let arg = evaluate(&args[0])?;
                if arg <= 0.0 {
                    return Err("Logarithm of non-positive number");
                }
                Ok(arg.ln())
            }
            ("log10", 1) => {
                let arg = evaluate(&args[0])?;
                if arg <= 0.0 {
                    return Err("Logarithm of non-positive number");
                }
                Ok(arg.log10())
            }
            ("log", 2) => {
                let base = evaluate(&args[0])?;
                let value = evaluate(&args[1])?;
                if base <= 0.0 || base == 1.0 || value <= 0.0 {
                    return Err("Invalid logarithm base or value");
                }
                Ok(value.log(base))
            }
            ("pow", 2) => {
                let base = evaluate(&args[0])?;
                let exponent = evaluate(&args[1])?;
                Ok(base.powf(exponent))
            }
            ("sqrt", 1) => {
                let arg = evaluate(&args[0])?;
                if arg < 0.0 {
                    return Err("Square root of negative number");
                }
                Ok(arg.sqrt())
            }
            ("abs", 1) => {
                let arg = evaluate(&args[0])?;
                Ok(arg.abs())
            }
            ("sin", 1) => Ok(evaluate(&args[0])?.sin()),
            ("cos", 1) => Ok(evaluate(&args[0])?.cos()),
            ("tan", 1) => Ok(evaluate(&args[0])?.tan()),
            ("asin", 1) => Ok(evaluate(&args[0])?.asin()),
            ("acos", 1) => Ok(evaluate(&args[0])?.acos()),
            ("atan", 1) => Ok(evaluate(&args[0])?.atan()),
            ("exp", 1) => Ok(evaluate(&args[0])?.exp()),
            ("cbrt", 1) => {
                let arg = evaluate(&args[0])?;
                if arg < 0.0 {
                    return Err("Cube root of negative number");
                }
                Ok(arg.cbrt())
            }
            ("ceil", 1) => Ok(evaluate(&args[0])?.ceil()),
            ("floor", 1) => Ok(evaluate(&args[0])?.floor()),
            ("round", 1) => Ok(evaluate(&args[0])?.round()),
            ("trunc", 1) => Ok(evaluate(&args[0])?.trunc()),
            ("signum", 1) => Ok(evaluate(&args[0])?.signum()),
            ("factorial", 1) => {
                let arg = evaluate(&args[0])?;
                if arg < 0.0 || arg.fract() != 0.0 {
                    return Err("Factorial of negative or non-integer number");
                }
                let n = arg as u64;
                Ok((1..=n).product::<u64>() as f64)
            }
            _ => Err("Unknown function"),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Expression;

    #[test]
    fn test_numeric_literal() {
        let expr = Expression::NumericLiteral(42.0);
        assert_eq!(evaluate(&expr).unwrap(), 42.0);
    }

    #[test]
    fn test_addition() {
        let expr = Expression::Addition(
            Box::new(Expression::NumericLiteral(2.0)),
            Box::new(Expression::NumericLiteral(3.0)),
        );
        assert_eq!(evaluate(&expr).unwrap(), 5.0);
    }

    #[test]
    fn test_subtraction() {
        let expr = Expression::Subtraction(
            Box::new(Expression::NumericLiteral(10.0)),
            Box::new(Expression::NumericLiteral(3.0)),
        );
        assert_eq!(evaluate(&expr).unwrap(), 7.0);
    }

    #[test]
    fn test_unary_minus() {
        let expr = Expression::Minus(Box::new(Expression::NumericLiteral(5.0)));
        assert_eq!(evaluate(&expr).unwrap(), -5.0);
    }

    #[test]
    fn test_multiplication() {
        let expr = Expression::Multiplication(
            Box::new(Expression::NumericLiteral(4.0)),
            Box::new(Expression::NumericLiteral(5.0)),
        );
        assert_eq!(evaluate(&expr).unwrap(), 20.0);
    }

    #[test]
    fn test_division() {
        let expr = Expression::Division(
            Box::new(Expression::NumericLiteral(15.0)),
            Box::new(Expression::NumericLiteral(3.0)),
        );
        assert_eq!(evaluate(&expr).unwrap(), 5.0);
    }

    #[test]
    fn test_division_by_zero() {
        let expr = Expression::Division(
            Box::new(Expression::NumericLiteral(10.0)),
            Box::new(Expression::NumericLiteral(0.0)),
        );
        assert!(evaluate(&expr).is_err());
    }

    #[test]
    fn test_exponentiation() {
        let expr = Expression::Exponentiation(
            Box::new(Expression::NumericLiteral(2.0)),
            Box::new(Expression::NumericLiteral(3.0)),
        );
        assert_eq!(evaluate(&expr).unwrap(), 8.0);
    }

    #[test]
    fn test_complex_expression() {
        // Test ((2 + 2) - 1) = 3
        let expr = Expression::Subtraction(
            Box::new(Expression::Addition(
                Box::new(Expression::NumericLiteral(2.0)),
                Box::new(Expression::NumericLiteral(2.0)),
            )),
            Box::new(Expression::NumericLiteral(1.0)),
        );
        assert_eq!(evaluate(&expr).unwrap(), 3.0);
    }

    #[test]
    fn test_operator_precedence() {
        // Test 2 * 3 + 4 = 10 (not 14)
        let expr = Expression::Addition(
            Box::new(Expression::Multiplication(
                Box::new(Expression::NumericLiteral(2.0)),
                Box::new(Expression::NumericLiteral(3.0)),
            )),
            Box::new(Expression::NumericLiteral(4.0)),
        );
        assert_eq!(evaluate(&expr).unwrap(), 10.0);
    }

    #[test]
    fn test_exponentiation_precedence() {
        // Test 2 ^ 3 ^ 2 = 512 (right associative: 2 ^ (3 ^ 2))
        let expr = Expression::Exponentiation(
            Box::new(Expression::NumericLiteral(2.0)),
            Box::new(Expression::Exponentiation(
                Box::new(Expression::NumericLiteral(3.0)),
                Box::new(Expression::NumericLiteral(2.0)),
            )),
        );
        assert_eq!(evaluate(&expr).unwrap(), 512.0);
    }
}
