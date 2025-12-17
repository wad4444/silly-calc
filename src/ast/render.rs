use crate::ast::{
    nodes::{Expression, Operator},
    tokens::Token,
};

pub trait Render {
    fn render(&self, buf: &mut String);
}

impl Render for Token {
    fn render(&self, buf: &mut String) {
        match self {
            Token::Operator(op) => match op {
                Operator::Plus => buf.push_str("+"),
                Operator::Minus => buf.push_str("-"),
                Operator::Multiply => buf.push_str("*"),
                Operator::Divide => buf.push_str("/"),
            },
            Token::OpeningParenthesis => buf.push_str("("),
            Token::ClosingParenthesis => buf.push_str(")"),
            Token::NumberLiteral(num) => buf.push_str(&num.to_string()),
        }
    }
}

impl Render for Expression {
    fn render(&self, buf: &mut String) {
        match self {
            Expression::Literal(lit) => {
                Token::NumberLiteral(*lit).render(buf);
            }
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                left.as_ref().render(buf);
                buf.push(' ');
                Token::Operator(operator.clone()).render(buf);
                buf.push(' ');
                right.as_ref().render(buf);
            }
            Expression::Parenthesized(par) => {
                Token::OpeningParenthesis.render(buf);
                par.as_ref().render(buf);
                Token::ClosingParenthesis.render(buf);
            }
        }
    }
}
