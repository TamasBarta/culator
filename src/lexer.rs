use itertools::Itertools;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    OpeningParenthesis,
    ClosingParenthesis,
    Comma,
    NumericLiteral(String),
    Name(String),
    Symbol(char),
}

pub fn lex(input: impl Into<String>) -> Result<Vec<Token>, ()> {
    let input: String = input.into();

    let mut tokens: Vec<Token> = vec![];

    let mut iterator = input.chars().multipeek();
    let mut current = iterator.next();

    while let Some(char) = current {
        match char {
            number if number.is_numeric() => {
                let mut number_buffer = String::from(number);
                while let Some(char) = iterator.peek() {
                    // TODO reconsider supporting 0x000 and similar syntaxes
                    if char.is_numeric() || *char == '.' {
                        number_buffer.push(*char);
                        iterator.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::NumericLiteral(number_buffer));
            }
            name if name.is_alphabetic() => {
                let mut name_buffer = String::from(name);
                while let Some(char) = iterator.peek() {
                    if char.is_alphanumeric() {
                        name_buffer.push(*char);
                        iterator.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Name(name_buffer));
            }
            whitespace if whitespace.is_whitespace() => {}
            '(' => tokens.push(Token::OpeningParenthesis),
            ')' => tokens.push(Token::ClosingParenthesis),
            ',' => tokens.push(Token::Comma),
            '+' | '-' | '*' | '/' | '^' => tokens.push(Token::Symbol(char.to_owned())),
            _ => return Err(()),
        }
        current = iterator.next();
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::Token::*;
    use super::*;

    #[test]
    fn it_works() {
        let result = lex("2 ( 8 )  727 + 727(sata2n(66a6))");
        println!("{:?}", result);
        assert_eq!(
            result,
            Ok(vec![
                NumericLiteral("2".into()),
                OpeningParenthesis,
                NumericLiteral("8".into()),
                ClosingParenthesis,
                NumericLiteral("727".into()),
                Symbol('+'),
                NumericLiteral("727".into()),
                OpeningParenthesis,
                Name("sata2n".into()),
                OpeningParenthesis,
                NumericLiteral("66".into()),
                Name("a6".into()),
                ClosingParenthesis,
                ClosingParenthesis
            ])
        );
    }
}
