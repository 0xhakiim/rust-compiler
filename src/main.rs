mod ast;
use ast::{Ast, evaluator::Evaluator, lexer::Lexer, parser::Parser};

fn main() {
    let mut lexer = Lexer::new("(1+2)+3");
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    let mut parser = Parser::from_tokens(tokens);
    let mut ast = Ast::new();

    while let Some(statement) = parser.next_statement() {
        ast.add_statement(statement);
    }
    println!("AST: {:?}", ast);
    ast.visualize();
    let mut evaluator = Evaluator::new();
    let result = evaluator.evaluate(&ast);
    println!("Result: {}", result);
}
