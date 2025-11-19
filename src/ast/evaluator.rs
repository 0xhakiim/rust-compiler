use crate::ast::{
    Ast, AstBinaryExpression, AstBinaryOperatorKind, AstExpression, AstExpressionKind,
    AstNumberExpression, AstParenthesizedExpression, AstVisitor,
};
pub struct Evaluator {
    value: i64,
}
impl Evaluator {
    pub fn new() -> Self {
        Self { value: 0 }
    }
    pub fn evaluate(&mut self, ast: &Ast) -> i64 {
        ast.visit(self);
        self.value
    }
}
impl AstVisitor for Evaluator {
    fn visit_number(&mut self, number: &AstNumberExpression) {
        self.value = number.value;
    }
    fn visit_expression(&mut self, expression: &AstExpression) {
        match &expression.kind {
            AstExpressionKind::Number(number) => {
                self.visit_number(&AstNumberExpression { value: *number })
            }
            AstExpressionKind::Binary(binary) => {
                self.visit_binary_expression(&AstBinaryExpression {
                    left: binary.left.clone(),
                    right: binary.right.clone(),
                    operator: binary.operator.clone(),
                })
            }
            AstExpressionKind::Parenthesized(parenthesized) => {
                self.visit_parenthesized_expression(&AstParenthesizedExpression {
                    expression: parenthesized.expression.clone(),
                })
            }
            _ => panic!("Unsupported expression type"),
        }
    }
    fn visit_binary_expression(&mut self, binary_expression: &AstBinaryExpression) {
        self.visit_expression(&binary_expression.left);
        let left = self.value;
        self.visit_expression(&binary_expression.right);
        let right = self.value;
        self.value = match binary_expression.operator.kind {
            AstBinaryOperatorKind::Add => left + right,
            AstBinaryOperatorKind::Subtract => left - right,
            AstBinaryOperatorKind::Multiply => left * right,
            AstBinaryOperatorKind::Divide => left / right,
        };
    }
    fn visit_parenthesized_expression(
        &mut self,
        parenthesized_expression: &AstParenthesizedExpression,
    ) {
        self.visit_expression(&parenthesized_expression.expression);
    }
}
