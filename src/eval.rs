use crate::ast::nodes::{Expression, Operator};

impl Expression {
    pub fn evaluate(&self) -> f64 {
        match self {
            Expression::Literal(lit) => *lit as f64,
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
