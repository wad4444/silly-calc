use crate::ast::nodes::{Expression, Operator};

pub trait Eval {
    fn evaluate(&self) -> i32;
}

impl Eval for Expression {
    fn evaluate(&self) -> i32 {
        match self {
            Expression::Literal(lit) => *lit,
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left_val = left.as_ref().evaluate();
                let right_val = right.as_ref().evaluate();
                match operator {
                    Operator::Plus => left_val + right_val,
                    Operator::Minus => left_val - right_val,
                    Operator::Multiply => left_val * right_val,
                    Operator::Divide => left_val / right_val,
                }
            }
            Expression::Parenthesized(par) => par.as_ref().evaluate(),
        }
    }
}