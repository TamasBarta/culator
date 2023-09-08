use crate::lexer::Token;

// 35% of 230
// Percentage(Box<Expression>, Box<Expression>),

#[derive(Debug, PartialEq)]
pub enum Expression {
    NumericLiteral(f64),
    Minus(Box<Expression>),
    Subtraction(Box<Expression>, Box<Expression>),
    Addition(Box<Expression>, Box<Expression>),
    FunctionCall(String, Vec<Expression>),
}

#[derive(Debug, PartialEq)]
enum TopLevelAtomic {
    Single {
        index: usize,
    },
    ParenthesisGroup {
        start_inclusive: usize,
        end_inclusive: usize,
    },
}

pub fn parse(tokens: &[Token]) -> Result<Expression, &'static str> {
    if tokens.is_empty() {
        return Err("Empty input");
    }

    let mut tokens = tokens;

    while start_and_ends_with_parenthesis(tokens) {
        tokens = &tokens[1..tokens.len() - 1];
    }

    if tokens.len() == 1 {
        match &tokens[0] {
            Token::NumericLiteral(number) => {
                return Ok(Expression::NumericLiteral(number.parse().unwrap()))
            }
            _ => return Err("Single token, but not a numeric literal."),
        }
    }

    let top_level_atoms = group_top_level_items(tokens)?;

    if let (
        2,
        TopLevelAtomic::Single { index },
        TopLevelAtomic::ParenthesisGroup {
            start_inclusive,
            end_inclusive,
        },
    ) = (
        top_level_atoms.len(),
        &top_level_atoms[0],
        &top_level_atoms[1],
    ) {
        let function_name_token = &tokens[*index];
        if let Token::Name(function_name) = function_name_token {
            let function_parameters_tokens = &tokens[*start_inclusive..*end_inclusive];
            let function_level_atoms = group_top_level_items(function_parameters_tokens)?;
            let valami = function_level_atoms
                .split(|a| match a {
                    &TopLevelAtomic::Single { index } => {
                        function_parameters_tokens[index] == Token::Comma
                    }
                    _ => false,
                })
                .flat_map(|function_parameter_atoms| {
                    let start = function_parameter_atoms[0].start_inclusive();
                    let end = function_parameter_atoms[function_parameter_atoms.len() - 1]
                        .start_inclusive();
                    let tokens_slice = &function_parameters_tokens[start..end];
                    parse(tokens_slice)
                });
        }
    }

    Err("Implementation isn't finished yet.")
}

impl TopLevelAtomic {
    fn start_inclusive(&self) -> usize {
        match self {
            TopLevelAtomic::Single { index } => *index,
            TopLevelAtomic::ParenthesisGroup {
                start_inclusive, ..
            } => *start_inclusive,
        }
    }

    fn end_inclusive(&self) -> usize {
        match self {
            TopLevelAtomic::Single { index } => *index,
            TopLevelAtomic::ParenthesisGroup { end_inclusive, .. } => *end_inclusive,
        }
    }
}

fn group_top_level_items(tokens: &[Token]) -> Result<Vec<TopLevelAtomic>, &'static str> {
    let mut top_level_atoms: Vec<TopLevelAtomic> = vec![];
    let mut parenthesis_level = 0;

    let mut parenthesis_start = 0;

    #[allow(clippy::needless_range_loop)]
    for i in 0..tokens.len() {
        let token = &tokens[i];
        match token {
            Token::OpeningParenthesis => {
                if parenthesis_level == 0 {
                    parenthesis_level += 1;
                    parenthesis_start = i;
                }
            }
            Token::ClosingParenthesis => {
                if parenthesis_level == 0 {
                    return Err("Unexpected closing parenthesis.");
                }
                parenthesis_level -= 1;
                if parenthesis_level == 0 {
                    let new_group = TopLevelAtomic::ParenthesisGroup {
                        start_inclusive: parenthesis_start,
                        end_inclusive: i - 1,
                    };
                    top_level_atoms.push(new_group)
                }
            }
            _ => {
                if parenthesis_level == 0 {
                    top_level_atoms.push(TopLevelAtomic::Single { index: i });
                }
            }
        }
    }

    if parenthesis_level != 0 {
        return Err("Mismatched parenthesis");
    }

    Ok(top_level_atoms)
}

fn start_and_ends_with_parenthesis(tokens: &[Token]) -> bool {
    let Some(last) = tokens.last() else {
        return false;
    };
    tokens[0] == Token::OpeningParenthesis && *last == Token::ClosingParenthesis
}
