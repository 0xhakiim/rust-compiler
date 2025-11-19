use super::{AstBinaryExpression, AstExpression, AstStatement, AstStatementKind};
use crate::ast::lexer::{AstToken, AstTokenKind};
use crate::ast::{AstBinaryOperator, AstBinaryOperatorKind, AstParenthesizedExpression};
pub struct Parser {
    pub(crate) tokens: Vec<AstToken>,
    current: usize,
}
impl Parser {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            current: 0,
        }
    }
    pub fn from_tokens(tokens: Vec<AstToken>) -> Self {
        Self { tokens, current: 0 }
    }
    pub fn next_statement(&mut self) -> Option<AstStatement> {
        let stmtm = self.parse_statement();
        if stmtm.is_some() {
            self.current += 1;
        }
        stmtm
    }
    pub fn parse_statement(&mut self) -> Option<AstStatement> {
        let token = self.current()?;
        if token.kind == AstTokenKind::EOF {
            return None;
        }
        let expr = self.parse_expression()?;

        Some(AstStatement::new(AstStatementKind::Expression(expr)))
    }
    pub fn parse_expression(&mut self) -> Option<AstExpression> {
        return self.parse_binary_expression(0);
    }

    pub fn parse_primary_expression(&mut self) -> Option<AstExpression> {
        let token = self.consume()?;
        match &token.kind {
            AstTokenKind::Number(value) => Some(AstExpression::number(*value)),
            AstTokenKind::LParen => {
                let expr = self.parse_expression()?;
                let token = self.consume()?;
                if token.kind != AstTokenKind::RParen {
                    panic!("Expected right paren!!!!!!!")
                }
                Some(AstExpression::parenthesized(expr))
            }
            _ => None,
        }
    }
    pub fn parse_binary_operator(&mut self) -> Option<AstBinaryOperator> {
        let token = self.current()?;
        match &token.kind {
            AstTokenKind::Plus => Some(AstBinaryOperator::new(AstBinaryOperatorKind::Add, 1)),
            AstTokenKind::Minus => Some(AstBinaryOperator::new(AstBinaryOperatorKind::Subtract, 1)),
            AstTokenKind::Star => Some(AstBinaryOperator::new(AstBinaryOperatorKind::Multiply, 2)),
            AstTokenKind::Slash => Some(AstBinaryOperator::new(AstBinaryOperatorKind::Divide, 2)),
            _ => None,
        }
    }
    pub fn parse_binary_expression(&mut self, precedence: u8) -> Option<AstExpression> {
        let mut left = self.parse_primary_expression()?;
        while let Some(operator) = self.parse_binary_operator() {
            self.consume();
            if operator.precedence < precedence {
                break;
            }
            let right = self.parse_binary_expression(operator.precedence)?;
            left = AstExpression::binary(left, right, operator);
        }
        Some(left)
    }
    pub fn peek(&self, offset: isize) -> Option<&AstToken> {
        self.tokens.get((self.current as isize + offset) as usize)
    }
    pub fn current(&mut self) -> Option<&AstToken> {
        self.peek(0)
    }
    fn consume(&mut self) -> Option<&AstToken> {
        self.current += 1;
        let token = self.peek(-1)?;
        return Some(token);
    }
}
