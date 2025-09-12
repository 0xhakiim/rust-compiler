pub mod lexer;
#[derive(Debug)]
pub struct Ast{
    pub statements: Vec<AstStatement>,
}
impl Ast{
    pub fn new() -> Self{
        Self { statements:Vec::new()}
    }
    pub fn addstatement(&mut self, statement: AstStatement){
        self.statements.push(statement);
    }
    pub fn visualize(&self){
        println!("digraph ast{{");
        println!("   node [shapre=box]");
        for (i,statement) in self.statements.iter().enumerate(){
            println!("Statement {}: {:?}", i, statement );
        }
        println!("}}");
    }
}
#[derive(Debug)]
pub struct AstStatement{
    pub kind: AstStatementKind,
}
#[derive(Debug)]
pub enum AstStatementKind{
    Expression(AstExpression),
}

impl AstStatement{
    pub fn new(kind: AstStatementKind) -> Self{
        Self { kind }
    }
}
#[derive(Debug)]
pub struct AstExpression{
    pub kind: AstExpressionKind,
}
#[derive(Debug)]
pub enum AstExpressionKind{
    Number(i64),
}
/*pub enum AstBinaryOperator{
    Add,
    Subtract,
    Multiply,
    Divide,
}*/
impl AstExpression{
    pub fn new(kind: AstExpressionKind) -> Self{
        Self{ kind }
    }

    pub fn visualize(&self){
        match &self.kind{
            AstExpressionKind::Number(value) => {
                println!("Number: {}", value);
            }
        }
    }
}
pub struct AstNumberExpression{
    pub value: i64,
}
trait AstVisitor{
    fn visit_expression(&mut self, expr: &AstExpression);
    fn visit_statement(&mut self, stmt: &AstStatement);
    fn visit_number(&mut self,num:&AstNumberExpression);
}