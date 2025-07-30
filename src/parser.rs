use crate::lexer::Token;

// 35% of 230
// Percentage(Box<Expression>, Box<Expression>),

#[derive(Debug, PartialEq)]
pub enum Expression {
    NumericLiteral(f64),
    Minus(Box<Expression>),
    Subtraction(Box<Expression>, Box<Expression>),
    Addition(Box<Expression>, Box<Expression>),
    Multiplication(Box<Expression>, Box<Expression>),
    Division(Box<Expression>, Box<Expression>),
    Exponentiation(Box<Expression>, Box<Expression>),
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

    // Check for function calls: Name followed by parentheses
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
            let function_parameters_tokens = &tokens[*start_inclusive + 1..*end_inclusive];
            if function_parameters_tokens.is_empty() {
                return Ok(Expression::FunctionCall(function_name.clone(), vec![]));
            }

            let function_level_atoms = group_top_level_items(function_parameters_tokens)?;
            let parameters: Result<Vec<Expression>, &'static str> = function_level_atoms
                .split(|a| match a {
                    &TopLevelAtomic::Single { index } => {
                        function_parameters_tokens[index] == Token::Comma
                    }
                    _ => false,
                })
                .map(|function_parameter_atoms| {
                    let start = function_parameter_atoms[0].start_inclusive();
                    let end = function_parameter_atoms[function_parameter_atoms.len() - 1]
                        .end_inclusive();
                    let tokens_slice = &function_parameters_tokens[start..=end];
                    parse(tokens_slice)
                })
                .collect();

            return Ok(Expression::FunctionCall(function_name.clone(), parameters?));
        }
    }

    // Check for binary operations with proper precedence and associativity
    
    // Lowest precedence: + and - (left to right)
    for (i, atom) in top_level_atoms.iter().enumerate().rev() {
        if let TopLevelAtomic::Single { index } = atom {
            match &tokens[*index] {
                Token::Symbol('+') => {
                    let left_tokens = extract_tokens_from_atoms(&top_level_atoms[..i], tokens);
                    let right_tokens = extract_tokens_from_atoms(&top_level_atoms[i + 1..], tokens);
                    let left = parse(&left_tokens)?;
                    let right = parse(&right_tokens)?;
                    return Ok(Expression::Addition(Box::new(left), Box::new(right)));
                }
                Token::Symbol('-') => {
                    // Improved unary minus detection: at the start, or after an operator or parenthesis
                    let is_unary = if i == 0 {
                        true
                    } else {
                        // Look at previous atom's end token
                        let prev_atom = &top_level_atoms[i - 1];
                        let prev_idx = prev_atom.end_inclusive();
                        match &tokens[prev_idx] {
                            Token::Symbol(_) | Token::OpeningParenthesis | Token::Comma => true,
                            _ => false,
                        }
                    };
                    if is_unary {
                        let right_atoms = &top_level_atoms[i + 1..];
                        let right_tokens = extract_tokens_from_atoms(right_atoms, tokens);
                        let right = parse(&right_tokens)?;
                        return Ok(Expression::Minus(Box::new(right)));
                    } else {
                        let left_tokens = extract_tokens_from_atoms(&top_level_atoms[..i], tokens);
                        let right_tokens =
                            extract_tokens_from_atoms(&top_level_atoms[i + 1..], tokens);
                        let left = parse(&left_tokens)?;
                        let right = parse(&right_tokens)?;
                        return Ok(Expression::Subtraction(Box::new(left), Box::new(right)));
                    }
                }
                _ => {}
            }
        }
    }

    // Medium precedence: * and / (left to right) 
    for (i, atom) in top_level_atoms.iter().enumerate().rev() {
        if let TopLevelAtomic::Single { index } = atom {
            match &tokens[*index] {
                Token::Symbol('*') | Token::Symbol('/') => {
                    let left_tokens = extract_tokens_from_atoms(&top_level_atoms[..i], tokens);
                    // Instead of extracting tokens only from the next atom, always use the full right slice for recursion
                    let right_start = top_level_atoms.get(i + 1).map(|a| a.start_inclusive()).unwrap_or(tokens.len());
                    let right_tokens = if right_start < tokens.len() {
                        tokens[right_start..].to_vec()
                    } else {
                        vec![]
                    };
                    let left = parse(&left_tokens)?;
                    let right = parse(&right_tokens)?;
                    let result = match &tokens[*index] {
                        Token::Symbol('*') => Expression::Multiplication(Box::new(left), Box::new(right)),
                        Token::Symbol('/') => Expression::Division(Box::new(left), Box::new(right)),
                        _ => unreachable!(),
                    };
                    return Ok(result);
                }
                _ => {}
            }
        }
    }

    // Highest precedence: ^ (right to left - find leftmost operator for right associativity)
    for (i, atom) in top_level_atoms.iter().enumerate() {
        if let TopLevelAtomic::Single { index } = atom {
            match &tokens[*index] {
                Token::Symbol('^') => {
                    let left_tokens = extract_tokens_from_atoms(&top_level_atoms[..i], tokens);
                    let right_tokens = extract_tokens_from_atoms(&top_level_atoms[i + 1..], tokens);
                    let left = parse(&left_tokens)?;
                    let right = parse(&right_tokens)?;
                    return Ok(Expression::Exponentiation(Box::new(left), Box::new(right)));
                }
                _ => {}
            }
        }
    }

    Err("Unable to parse expression")
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
                    parenthesis_start = i;
                }
                parenthesis_level += 1;
            }
            Token::ClosingParenthesis => {
                if parenthesis_level == 0 {
                    return Err("Unexpected closing parenthesis.");
                }
                parenthesis_level -= 1;
                if parenthesis_level == 0 {
                    let new_group = TopLevelAtomic::ParenthesisGroup {
                        start_inclusive: parenthesis_start,
                        end_inclusive: i,
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

fn extract_tokens_from_atoms(atoms: &[TopLevelAtomic], tokens: &[Token]) -> Vec<Token> {
    if atoms.is_empty() {
        return vec![];
    }

    let start = atoms[0].start_inclusive();
    let end = atoms[atoms.len() - 1].end_inclusive();
    tokens[start..=end].to_vec()
}

fn start_and_ends_with_parenthesis(tokens: &[Token]) -> bool {
    let Some(last) = tokens.last() else {
        return false;
    };
    tokens[0] == Token::OpeningParenthesis && *last == Token::ClosingParenthesis
}
