use crate::ast::nodes::Operator;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Token {
    Operator(Operator),
    OpeningParenthesis,
    ClosingParenthesis,
    NumberLiteral(i32),
}

impl Token {
    pub fn to_operator(&self) -> Option<Operator> {
        if let Token::Operator(op) = self {
            Some(*op)
        } else {
            None
        }
    }
}
