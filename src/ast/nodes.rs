#[derive(Clone, Debug)]
pub enum Expression {
    Literal(i32),
    Binary {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    Parenthesized(Box<Expression>),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}
