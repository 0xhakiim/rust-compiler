#[derive(Debug, Clone, PartialEq)]
pub struct AstToken {
    pub(crate) kind: AstTokenKind,
    pub(crate) span: Span,
}
impl AstToken {
    fn new(kind: AstTokenKind, span: Span) -> Self {
        AstToken { kind, span }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum AstTokenKind {
    Identifier(String),
    Number(i64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    EOF,
    BAD,
    // Add more token kinds as needed
}
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) literal: String,
}
impl Span {
    fn new(start: usize, end: usize, literal: String) -> Self {
        Span {
            start,
            end,
            literal,
        }
    }
    fn length(&self) -> usize {
        self.end - self.start
    }
}
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}
impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, position: 0 }
    }
    pub fn next_token(&mut self) -> Option<AstToken> {
        // Implement tokenization logic here
        if self.position == self.input.len() {
            self.position += 1;
            return Some(AstToken::new(
                AstTokenKind::EOF,
                Span::new(self.position, self.position, "\0".to_string()),
            ));
        }
        if self.position > self.input.len() {
            return None;
        }
        let kind;
        let start = self.position;
        if self.is_number_start() {
            let number = self.consume_number();
            kind = AstTokenKind::Number(number);
        } else {
            kind = self.consume_ponctuation();
        }
        let end = self.position;
        let literal = self.input[start..end].to_string();
        let span = Span::new(start, end, literal);
        Some(AstToken::new(kind, span))
    }
    pub fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }
    fn is_number_start(&mut self) -> bool {
        match self.peek() {
            Some(c) => c.is_digit(10),
            _ => false,
        }
    }
    fn consume(&mut self) -> Option<char> {
        let ch = self.peek();
        if ch.is_some() {
            self.position += 1;
        }
        ch
    }
    fn consume_ponctuation(&mut self) -> AstTokenKind {
        let ch = self.consume().unwrap();
        return match ch {
            '+' => AstTokenKind::Plus,
            '-' => AstTokenKind::Minus,
            '*' => AstTokenKind::Star,
            '/' => AstTokenKind::Slash,
            '(' => AstTokenKind::LParen,
            ')' => AstTokenKind::RParen,
            _ => return AstTokenKind::BAD,
        };
    }
    pub fn consume_number(&mut self) -> i64 {
        let mut number: i64 = 0;
        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                number = number * 10 + c.to_digit(10).unwrap() as i64;
                self.consume();
            } else {
                break;
            }
        }
        number
    }
}
