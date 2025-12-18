use crate::{
    ast::{nodes::Operator, tokens::Token},
    parse::error::ErrorKind,
};

pub struct Tokenizer<'a> {
    input: &'a String,
    tokens: Vec<Token>,
    errors: Vec<ErrorKind<'a>>,
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a String) -> Tokenizer {
        Tokenizer {
            input,
            tokens: Vec::new(),
            errors: Vec::new(),
            pos: 0,
        }
    }

    fn current(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }

    fn next(&mut self) -> Option<char> {
        self.pos += 1;
        self.current()
    }

    fn number_literal(&mut self) {
        let mut number_str = String::new();

        while let Some(char) = self.current() {
            if !char.is_ascii_digit() {
                break;
            }

            number_str.push(char);
            self.next();
        }

        let end = self.pos - 1;
        let result = number_str.parse::<i32>();

        self.pos -= 1;
        if result.is_err() {
            let start = end - number_str.len();
            self.errors.push(ErrorKind::InvalidNumber {
                contents: &self.input[start..end + 1],
                start,
                end,
            });
            return;
        }

        self.tokens.push(Token::NumberLiteral(result.unwrap()));
    }

    fn reset(&mut self) {
        self.pos = 0;
        self.tokens.clear();
        self.errors.clear();
    }

    pub fn tokenize(&mut self) -> (Vec<Token>, Vec<ErrorKind<'a>>) {
        self.reset();

        while let Some(char) = self.current() {
            match char {
                '+' => self.tokens.push(Token::Operator(Operator::Plus)),
                '-' => self.tokens.push(Token::Operator(Operator::Minus)),
                '*' => self.tokens.push(Token::Operator(Operator::Multiply)),
                '/' => self.tokens.push(Token::Operator(Operator::Divide)),
                '(' => self.tokens.push(Token::OpeningParenthesis),
                ')' => self.tokens.push(Token::ClosingParenthesis),
                char if char.is_ascii_digit() => self.number_literal(),
                char if char.is_whitespace() => {}
                _ => self.errors.push(ErrorKind::InvalidSymbol {
                    contents: &self.input[self.pos..self.pos + 1],
                    pos: self.pos,
                }),
            }
            self.next();
        }

        (self.tokens.to_vec(), self.errors.to_vec())
    }
}
