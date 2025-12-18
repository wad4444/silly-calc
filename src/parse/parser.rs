use crate::{
    ast::{
        nodes::{Expression, Operator},
        tokens::Token,
    },
    parse::error::ErrorKind,
};

pub struct Parser<'a> {
    input: &'a Vec<Token>,
    errors: Vec<ErrorKind<'a>>,
    pos: usize,
}

impl Parser<'_> {
    pub fn new(input: &Vec<Token>) -> Parser {
        Parser {
            input,
            errors: Vec::new(),
            pos: 0,
        }
    }

    fn current(&self) -> Option<&Token> {
        self.input.get(self.pos)
    }

    fn reset(&mut self) {
        self.pos = 0;
        self.errors.clear();
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        let expression = self.parse_term();
        if expression.is_none() {
            return None;
        }

        let mut expression = expression.unwrap();
        while let Some(token) = self.current() {
            let token = *token;
            let is_divide_or_multiply = token == Token::Operator(Operator::Plus)
                || token == Token::Operator(Operator::Minus);

            if !is_divide_or_multiply {
                break;
            }

            self.pos += 1;
            let factor_expr = self.parse_term();
            if factor_expr.is_none() {
                return None;
            }

            expression = Expression::Binary {
                left: Box::new(expression),
                operator: token.to_operator().unwrap(),
                right: Box::new(factor_expr.unwrap()),
            }
        }

        Some(expression)
    }

    fn parse_term(&mut self) -> Option<Expression> {
        let expression = self.parse_factor();
        if expression.is_none() {
            return None;
        }

        let mut expression = expression.unwrap();
        while let Some(token) = self.current() {
            let token = *token;
            let is_divide_or_multiply = token == Token::Operator(Operator::Multiply)
                || token == Token::Operator(Operator::Divide);

            if !is_divide_or_multiply {
                break;
            }

            self.pos += 1;
            let factor_expr = self.parse_factor();
            if factor_expr.is_none() {
                return None;
            }

            expression = Expression::Binary {
                left: Box::new(expression),
                operator: token.to_operator().unwrap(),
                right: Box::new(factor_expr.unwrap()),
            }
        }

        Some(expression)
    }

    fn parse_factor(&mut self) -> Option<Expression> {
        let current = self.current();
        if current.is_none() {
            self.unexpected_end(Some(Token::NumberLiteral(0)));
            return None;
        }

        let current = *current.unwrap();
        match current {
            Token::NumberLiteral(literal) => {
                self.pos += 1;
                Some(Expression::Literal(literal))
            }
            Token::OpeningParenthesis => {
                self.pos += 1;

                let expression = self.parse_expression();
                if expression.is_none() {
                    return None;
                }

                if !self.ensure_current_tok(Token::ClosingParenthesis) {
                    return None;
                }

                self.pos += 1;
                Some(Expression::Parenthesized(Box::new(expression.unwrap())))
            }
            token => {
                self.errors.push(ErrorKind::UnexpectedToken {
                    got: token,
                    expected: None,
                    index: self.pos,
                });
                None
            }
        }
    }

    fn ensure_current_tok(&mut self, expected: Token) -> bool {
        let next = self.current();
        if next.is_none() {
            self.unexpected_end(Some(expected));
            return false;
        }

        let token = *next.unwrap();
        if token != expected {
            self.errors.push(ErrorKind::UnexpectedToken {
                got: token,
                expected: Some(expected),
                index: self.pos,
            });
            return false;
        }

        return true;
    }

    fn unexpected_end(&mut self, expected: Option<Token>) {
        self.errors.push(ErrorKind::UnexpectedEndOfExpression {
            expected,
            index: self.pos + 1,
        });
    }

    pub fn parse(&mut self) -> (Option<Expression>, Vec<ErrorKind>) {
        self.reset();

        let result = self.parse_expression();
        let next_token = self.current();
        if next_token.is_some() {
            self.errors.push(ErrorKind::UnexpectedToken {
                got: *next_token.unwrap(),
                expected: None,
                index: self.pos + 1,
            });
            return (None, self.errors.to_vec());
        }

        (result, self.errors.to_vec())
    }
}
