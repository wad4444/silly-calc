use crate::ast::{render::Render, tokens::Token};

#[derive(Clone, Debug)]
pub enum ErrorKind<'a> {
    InvalidSymbol {
        contents: &'a str,
        pos: usize,
    },
    InvalidNumber {
        contents: &'a str,
        start: usize,
        end: usize,
    },
    UnexpectedToken {
        got: Token,
        expected: Option<Token>,
        index: usize,
    },
    UnexpectedEndOfExpression {
        index: usize,
        expected: Option<Token>,
    },
}

impl Render for ErrorKind<'_> {
    fn render(&self, buf: &mut String) {
        match self {
            ErrorKind::UnexpectedToken {
                got,
                expected,
                index,
            } => {
                buf.push_str(&format!(
                    "Error: Unexpected token '{:?}' at index '{index}'. Expected '{:?}'",
                    got, expected
                ));
            }
            ErrorKind::InvalidSymbol { contents, pos } => buf.push_str(&format!(
                "Tokenizing Error: Invalid symbol '{contents}' at position '{pos}'",
            )),
            ErrorKind::InvalidNumber {
                contents,
                start,
                end,
            } => buf.push_str(&format!(
                "Tokenizing Error: Invalid number '{contents}'\nat positions {start}-{end}'",
            )),
            ErrorKind::UnexpectedEndOfExpression { expected, index } => {
                buf.push_str(&format!(
                    "Parsing Error: Unexpected end of expression at index {index}\nExpected '{:?}'",
                    expected
                ));
            }
        }
        buf.push('\n');
    }
}
