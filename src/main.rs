use crate::{ast::render::Render, eval::Eval, parse::{parser::Parser, tokenizer::Tokenizer}};

mod ast;
mod parse;
mod eval;

fn main() {
    let input = String::from("(10 + 20) * 2 / 3");

    // TODO: add proper error handling and user input

    let mut tokenizer = Tokenizer::new(&input);
    let (tokens, _) = tokenizer.tokenize();

    let mut parser = Parser::new(&tokens);
    let (ast, _) = parser.parse();

    let mut buf = String::new();
    let ast = ast.unwrap();
    ast.render(&mut buf);

    println!("AST Representation:\n{}", buf);
    println!("{}", ast.evaluate());
}
