pub mod evaluator;
pub mod lexer;
pub mod parser;
#[derive(Debug, PartialEq, Clone)]
pub struct Ast {
    pub statements: Vec<AstStatement>,
}
impl Ast {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    pub fn add_statement(&mut self, statement: AstStatement) {
        self.statements.push(statement);
    }

    pub fn visit(&self, visitor: &mut dyn AstVisitor) {
        for statement in &self.statements {
            visitor.visit_statement(statement);
        }
    }

    pub fn visualize(&self) -> () {
        let mut printer = AstPrinter { indent: 0 };
        self.visit(&mut printer);
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct AstStatement {
    pub kind: AstStatementKind,
}
#[derive(Debug, PartialEq, Clone)]
pub enum AstStatementKind {
    Expression(AstExpression),
}

impl AstStatement {
    pub fn new(kind: AstStatementKind) -> Self {
        Self { kind }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct AstExpression {
    pub kind: AstExpressionKind,
}
#[derive(Debug, PartialEq, Clone)]
pub enum AstExpressionKind {
    Number(i64),
    Binary(AstBinaryExpression),
    Parenthesized(AstParenthesizedExpression),
}
#[derive(Debug, PartialEq, Clone)]
pub struct AstParenthesizedExpression {
    pub expression: Box<AstExpression>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct AstBinaryExpression {
    pub left: Box<AstExpression>,
    pub right: Box<AstExpression>,
    pub operator: AstBinaryOperator,
}
#[derive(Debug, PartialEq, Clone)]
pub struct AstBinaryOperator {
    pub kind: AstBinaryOperatorKind,
    pub precedence: u8,
}
impl AstBinaryOperator {
    pub fn new(kind: AstBinaryOperatorKind, precedence: u8) -> Self {
        Self { kind, precedence }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum AstBinaryOperatorKind {
    Add,
    Subtract,
    Multiply,
    Divide,
}
impl AstExpression {
    pub fn new(kind: AstExpressionKind) -> Self {
        Self { kind }
    }
    pub fn number(value: i64) -> Self {
        Self {
            kind: AstExpressionKind::Number(value),
        }
    }
    pub fn binary(left: AstExpression, right: AstExpression, operator: AstBinaryOperator) -> Self {
        Self {
            kind: AstExpressionKind::Binary(AstBinaryExpression {
                left: Box::new(left),
                right: Box::new(right),
                operator,
            }),
        }
    }
    pub fn parenthesized(expression: AstExpression) -> Self {
        Self {
            kind: AstExpressionKind::Parenthesized(AstParenthesizedExpression {
                expression: Box::new(expression),
            }),
        }
    }
}

pub struct AstNumberExpression {
    pub value: i64,
}

pub trait AstVisitor {
    fn do_visit_statement(&mut self, statement: &AstStatement) {
        match &statement.kind {
            AstStatementKind::Expression(expr) => {
                self.visit_expression(expr);
            }
        }
    }
    fn visit_statement(&mut self, statement: &AstStatement) {
        self.do_visit_statement(statement);
    }
    fn do_visit_expression(&mut self, expression: &AstExpression) {
        match &expression.kind {
            AstExpressionKind::Number(number) => {
                self.visit_number(&AstNumberExpression { value: *number });
            }
            AstExpressionKind::Binary(expr) => {
                self.visit_binary_expression(expr);
            }
            AstExpressionKind::Parenthesized(expr) => {
                self.visit_parenthesized_expression(expr);
            }
        }
    }
    fn visit_expression(&mut self, expression: &AstExpression) {
        self.do_visit_expression(expression);
    }

    fn visit_number(&mut self, number: &AstNumberExpression);

    fn visit_binary_expression(&mut self, binary_expression: &AstBinaryExpression) {
        self.visit_expression(&binary_expression.left);
        self.visit_expression(&binary_expression.right);
    }

    fn visit_parenthesized_expression(
        &mut self,
        parenthesized_expression: &AstParenthesizedExpression,
    ) {
        self.visit_expression(&parenthesized_expression.expression);
    }
}

pub struct AstPrinter {
    indent: usize,
}
const LEVEL_INDENT: usize = 2;

impl AstVisitor for AstPrinter {
    fn visit_statement(&mut self, statement: &AstStatement) {
        self.print_with_indent("Statement:");
        self.indent += LEVEL_INDENT;
        AstVisitor::do_visit_statement(self, statement);
        self.indent -= LEVEL_INDENT;
    }

    fn visit_expression(&mut self, expression: &AstExpression) {
        self.print_with_indent("Expression:");
        self.indent += LEVEL_INDENT;
        AstVisitor::do_visit_expression(self, expression);
        self.indent -= LEVEL_INDENT;
    }

    fn visit_number(&mut self, number: &AstNumberExpression) {
        self.print_with_indent(&format!("Number: {}", number.value));
    }

    fn visit_binary_expression(&mut self, binary_expression: &AstBinaryExpression) {
        self.print_with_indent("Binary Expression:");
        self.indent += LEVEL_INDENT;
        self.print_with_indent(&format!("Operator: {:?}", binary_expression.operator.kind));
        self.visit_expression(&binary_expression.left);
        self.visit_expression(&binary_expression.right);
        self.indent -= LEVEL_INDENT;
    }

    fn visit_parenthesized_expression(
        &mut self,
        parenthesized_expression: &AstParenthesizedExpression,
    ) {
        self.print_with_indent("Parenthesized Expression:");
        self.indent += LEVEL_INDENT;
        self.visit_expression(&parenthesized_expression.expression);
        self.indent -= LEVEL_INDENT;
    }
}

impl AstPrinter {
    fn print_with_indent(&mut self, text: &str) {
        println!("{}{}", " ".repeat(self.indent), text);
    }
}
