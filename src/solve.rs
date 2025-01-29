use std::time::Duration;

use crate::*;

pub fn solve(tokens: parse::Tokens) -> Result<(Vec<f64>, Duration), SolverError> {
    let start = std::time::Instant::now();

    let mut stack = Vec::new();

    for token in tokens {
        match token {
            Ok(Token::Operand(num)) => stack.push(num),
            Ok(Token::Operator(operation)) => {
                if let (Some(rhs), Some(lhs)) = (stack.pop(), stack.pop()) {
                    stack.push(operation.solve(lhs, rhs));
                } else {
                    return Err(SolverError::StackUnderflow(operation));
                }
            }
            Err(err) => return Err(SolverError::TokenizerError(err)),
        }
    }

    Ok((stack, start.elapsed()))
}

pub enum SolverError {
    TokenizerError(parse::error::TokenizerError),
    StackUnderflow(parse::Operation),
}
impl std::fmt::Display for SolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SolverError::TokenizerError(tokenizer_error) => {
                write!(f, "tokenizer error: {}", tokenizer_error)
            }
            SolverError::StackUnderflow(operation) => {
                write!(f, "stack underflow during {:?}", operation)
            }
        }
    }
}

impl parse::Operation {
    pub fn solve(&self, lhs: f64, rhs: f64) -> f64 {
        match self {
            parse::Operation::Add => lhs + rhs,
            parse::Operation::Subtract => lhs - rhs,
            parse::Operation::Multiply => lhs * rhs,
            parse::Operation::Divide => lhs / rhs,
        }
    }
}