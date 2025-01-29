use std::str::FromStr;

use error::{ParseOperationError, TokenizerError};

#[derive(Debug)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}
impl Operation {
    pub fn solve(&self, lhs: f64, rhs: f64) -> f64 {
        match self {
            Operation::Add => lhs + rhs,
            Operation::Subtract => lhs - rhs,
            Operation::Multiply => lhs * rhs,
            Operation::Divide => lhs / rhs,
        }
    }
}
impl FromStr for Operation {
    type Err = error::ParseOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(ParseOperationError {
                lexeme: s.to_string(),
            });
        }

        match s.chars().next() {
            Some('+') => Ok(Operation::Add),
            Some('-') => Ok(Operation::Subtract),
            Some('*') => Ok(Operation::Multiply),
            Some('/') => Ok(Operation::Divide),

            _ => Err(ParseOperationError {
                lexeme: s.to_string(),
            }),
        }
    }
}

#[derive(Debug)]
pub enum Token {
    Operand(f64),
    Operator(Operation),
}

#[derive(Clone)]
pub struct Tokens {
    pub lexemes: std::vec::IntoIter<(usize, String)>,
}

impl Iterator for Tokens {
    type Item = Result<Token, error::TokenizerError>;

    fn next(&mut self) -> Option<Self::Item> {
        let lexeme = self.lexemes.next()?;

        let operation_result = lexeme.1.parse::<Operation>();
        if let Ok(operation) = operation_result {
            return Some(Ok(Token::Operator(operation)));
        }

        let operand_result = lexeme.1.parse::<f64>();
        if let Ok(operand) = operand_result {
            return Some(Ok(Token::Operand(operand)));
        }

        Some(Err(TokenizerError {
            line: lexeme.0,
            parse_operation_error: operation_result.unwrap_err(),
        }))
    }
}

impl Tokens {
    pub fn new(string: String) -> Self {
        // convert string into lexemes for the tokenizer iterator

        // Vec<(line_num, Vec<sequence>)>
        // line num is used for slightly better error printing, unfortunately i cant easily get column num though
        let lines_sequences = string
            .lines()
            .enumerate()
            .map(|(line_num, line)| (line_num + 1, line.split_whitespace().collect::<Vec<_>>()))
            .collect::<Vec<_>>();

        let mut lexemes = Vec::new();
        lexemes.push((0, "".to_string()));

        for (line_num, lines) in lines_sequences {
            for sequence in lines {
                for char in sequence.chars() {
                    if !(char.is_ascii_digit() | (char == '.')) {
                        lexemes.push((line_num, char.to_string()));
                    } else {
                        lexemes.last_mut().unwrap().1.push(char);
                    }
                }
                lexemes.push((line_num, "".to_string()));
            }
        }

        // the above step introduces empty "" lexemes sometimes, so we retain only non-empty lexemes
        lexemes.retain(|(_, s)| !s.is_empty());

        Self {
            lexemes: lexemes.into_iter(),
        }
    }
}

pub mod error {
    use std::fmt;

    #[derive(Debug)]
    pub struct TokenizerError {
        pub line: usize,
        pub parse_operation_error: ParseOperationError,
    }
    impl fmt::Display for TokenizerError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "line {}: {}", self.line, self.parse_operation_error)
        }
    }

    #[derive(Debug)]
    pub struct ParseOperationError {
        pub lexeme: String,
    }
    impl fmt::Display for ParseOperationError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "invalid operator `{}`", self.lexeme)
        }
    }
}
