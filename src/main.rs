use crate::{
    ast::render::Render,
    parse::{parser::Parser, tokenizer::Tokenizer},
};

mod ast;
mod eval;
mod parse;

fn main() {
    let input = String::from("5 * 5 + (10 - 2) / ");

    // TODO: add proper error handling and user input

    let mut tokenizer = Tokenizer::new(&input);
    let (tokens, tokenizing_errors) = tokenizer.tokenize();

    let mut parser = Parser::new(&tokens);
    let (ast, parsing_errors) = parser.parse();

    let mut buf = String::new();

    for error in tokenizing_errors {
        error.render(&mut buf);
    }

    for error in parsing_errors {
        error.render(&mut buf);
    }

    match ast {
        Some(ast) => {
            buf.push_str("\nAST: ");
            ast.render(&mut buf);
            buf.push_str("\nResult: ");

            let result = ast.evaluate().to_string();
            buf.push_str(&result);
        }
        None => {}
    }

    println!("{}", buf);
}
