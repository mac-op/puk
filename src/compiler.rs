use std::io;
use std::str;
use std::io::Write;
use std::string::String;
use crate::machine::InterpretResult;
use crate::op_code::{Chunk, OpCode, Value};
use crate::token::{IDENTIFIER_MAP, Token, TokenType};
use crate::token::TokenType::*;

pub struct Scanner<'a> {
    content: &'a [u8],
    start: usize,
    current: usize,
    line: i32,
}

pub struct Parser<'a> {
    curr_tok: Option<Token<'a>>,
    prev_tok: Option<Token<'a>>,
    error_occurred: bool,
    panic: bool
}

pub struct Compiler {
    // scanner: Scanner
}

impl<'a> Scanner<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            content: content.as_bytes(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub(crate) fn scan_tok(&mut self) -> Token {
        self.skip_white_space();
        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(EOF);
        }
        
        let c = self.advance() as char;

        if c.is_numeric() { return self.make_num(); }
        if is_alpha(c) { return self.make_identifier(); }
        match c {
            '(' => { return self.make_token(LeftParen); }
            ')' => { return self.make_token(RightParen); }
            '{' => { return self.make_token(LeftBrace); }
            '}' => { return self.make_token(RightBrace); }
            ',' => { return self.make_token(Comma); }
            ';' => { return self.make_token(Semicolon); }
            '-' => { return self.make_token(Minus); }
            '+' => { return self.make_token(Plus); }
            '*' => { return self.make_token(Star); }
            '/' => { return self.make_token(Slash); }
            '!' => {
                return if self.match_next('=') { self.make_token(BangEq) } else { self.make_token(Bang) }
            }
            '=' => {
                return if self.match_next('=') { self.make_token(EqEq) } else { self.make_token(Eq) }
            }
            '<' => {
                return if self.match_next('=') { self.make_token(LessEq) } else { self.make_token(Less) }
            }
            '>' => {
                return if self.match_next('=') { self.make_token(GreaterEq) } else { self.make_token(Greater) }
            }
            '"' => {
                return self.make_str();
            }
            _ => {}
        }
        self.make_err("Unexpected character")
    }

    fn make_token(&self, tok_type: TokenType) -> Token {
        Token::new(tok_type, &self.content[self.start..self.current+1]/*, self.current - self.start*/, self.line)
    }

    fn make_err(&self, msg: &'static str) -> Token {
        Token::new(Err, msg.as_bytes()/*, self.current - self.start*/, self.line)
    }

    fn is_at_end(&self) -> bool {
        self.current == self.content.len() - 1
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false; }
        if self.content[self.current] as char != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.content[self.current - 1]
    }

    fn skip_white_space(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => { self.current += 1; }
                '\n' => { self.line += 1; self.current += 1; }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek_next() != '\n' && !self.is_at_end() {
                            self.current += 1;
                        }
                    } else {
                        return;
                    }
                }
                _ => { return; }
            }
        }
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() { return '\0'; }
        self.content[self.current + 1] as char
    }

    fn peek(&self) -> char { self.content[self.current] as char }

    fn make_str(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line += 1; }
            self.current += 1;
        }
        if self.is_at_end() {
            return self.make_err("Unterminated string");
        }
        self.current += 1;
        self.make_token(String)
    }

    fn make_num(&mut self) -> Token {
        while self.peek().is_numeric() {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.advance();
            while self.peek().is_numeric() { self.advance(); }
        }
        self.make_token(Number)
    }

    fn make_identifier(&mut self) -> Token {
        while is_alpha(self.peek()) || self.peek().is_numeric() { self.advance(); }

        self.make_token(Var)
    }

    fn get_id_type(&self) -> TokenType {
        let token = &self.content[self.start..self.current+1];
        if let Some(tok_type) = IDENTIFIER_MAP.iter()
            .find(|(id, _)| *id == str::from_utf8(token).unwrap()) {
            return tok_type.1;
        }
        Var
    }

    pub fn consume(&mut self, parser: &mut Parser, tok_type: TokenType, msg: &'static str) {
        if matches!(parser.curr_tok.as_ref().unwrap().tok_type, tok_type) {
            self.advance();
            return;
        }
        parser.error_at_curr(msg);
    }

    pub fn unary(&mut self, parser: &mut Parser) {
        let op_type =  parser.prev_tok.as_ref().unwrap().tok_type;

        // expression()
        if matches!(op_type, Minus) {
            parser.emit_byte()
        }
    }

    pub fn grouping(&mut self, parser:&mut Parser) {
        // expression()
        self.consume(parser, RightParen, "Expect ')' after expression.");
    }

}

impl Parser<'_> {

    pub fn error_at_curr(&mut self, msg: &'static str) {
        let tok = self.curr_tok.as_ref().unwrap();
        self.error_at(true, msg);
    }

    pub fn error(&mut self, msg: &'static str) {
        self.error_at(false, msg);
    }

    fn error_at(&mut self, current_tok: bool, msg: &'static str) {
        if self.panic { return; }
        self.panic = true;
        let tok = if current_tok {self.curr_tok.as_ref().unwrap()} else {self.prev_tok.as_ref().unwrap()};
        eprint!("[line {}] Error", tok.line);
        match tok.tok_type{
            EOF => { eprint!(" at end"); }
            Err => {}
            _ => { eprint!(" at {}", str::from_utf8(tok.content).unwrap()); }
        }

        eprintln!(": {msg}");
        self.error_occurred = true;
    }


    pub fn emit_byte(&self, chunk: &mut Chunk, op_code: OpCode) {
        chunk.add_instr(op_code, self.prev_tok.as_ref().unwrap().line);
    }

    pub fn emit_constant(&self, chunk: &mut Chunk, value: Value) {
        chunk.add_constant(value, self.prev_tok.as_ref().unwrap().line);
    }

    pub fn add_number(&self, chunk: &mut Chunk) {
        let num = str::from_utf8(self.prev_tok.as_ref().unwrap().content)
            .unwrap().parse::<f64>().unwrap();
        self.emit_constant(chunk, num);
    }

}

impl Compiler {
    pub fn compile(&mut self, content: &str, chunk: &mut Chunk) -> InterpretResult {
        let mut scanner: Scanner = Scanner::new(content);
        let mut parser = Parser{
            curr_tok: None,
            prev_tok: None,
            error_occurred: false,
            panic: false
        };
        // let mut line: i32 = -1;
        // loop {
        //     let tok: Token = scanner.scan_tok();
        //     if tok.line != line {
        //         print!("{:>4} ", tok.line);
        //         line = tok.line;
        //     } else {
        //         print!("   ^ ");
        //     }
        //     println!("{:2} '{}'", tok.tok_type as u32,
        //              String::from_utf8_lossy(&tok.content));
        //     if matches!(tok.tok_type, EOF) {
        //         break;
        //     }
        //     io::stdout().flush().unwrap();
        // }
        scanner.advance();
        //expression();
        scanner.consume(&mut parser, EOF, "Expect end of expression");
        parser.emit_byte(chunk, OpCode::Return);
        return if parser.error_occurred {InterpretResult::CompileErr} else {InterpretResult::OK};
    }

}

fn is_alpha(c: char) -> bool { c.is_alphabetic() || c == '_' }
