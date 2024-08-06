use std::collections::HashMap;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum TokenType {
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Semicolon, Minus, Plus, Star, Slash,
    Bang, BangEq, Eq, EqEq,
    Greater, GreaterEq, Less, LessEq,
    Identifier, String, Number,
    Var, Class, Super, This,
    And, Or, If, Else, True, False, For, While,
    Func, Nil, Print, Return,
    Err, EOF
}

pub(crate) static IDENTIFIER_MAP: [(&str, TokenType); 16] = [
    ("and",     TokenType::And),
    ("class",   TokenType::Class),
    ("else",    TokenType::Else),
    ("if",      TokenType::If),
    ("nil",     TokenType::Nil),
    ("or",      TokenType::Or),
    ("print",   TokenType::Print),
    ("return",  TokenType::Return),
    ("super",   TokenType::Super),
    ("this",    TokenType::This),
    ("true",    TokenType::True),
    ("false",   TokenType::False),
    ("for",     TokenType::For),
    ("while",   TokenType::While),
    ("var",     TokenType::Var),
    ("func",    TokenType::Func),
];

pub struct Token<'a> {
    pub tok_type: TokenType,
    pub content: &'a [u8],
    // pub length: usize,
    pub line: i32,
}

impl<'a> Token<'a> {
    pub fn new(tok_type: TokenType, start: &'a [u8], /*length: usize,*/ line: i32) -> Self {
        Self {
            tok_type,
            content: start,
            // length,
            line,
        }
    }
}