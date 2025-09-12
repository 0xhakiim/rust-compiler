mod ast;
use ast::{Ast,lexer::Lexer};

use crate::ast::lexer::AstTokenKind;

fn main() {
    let ast  =Ast::new();
    let mut lexer =Lexer::new("7");
    while let Some(token) = lexer.next_token(){
        println!("{:?}", token);
        if let AstTokenKind::EOF = token.kind{
            break;
        }
    }
    


}
