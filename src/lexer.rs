use std::{fs::read_to_string};

#[derive(Debug)]
pub enum Token {
    Ident(String),
    Keyword(KeywordType),
    Literal(String),
    Op(OpType),
    Invalid(char),
}

#[derive(Debug, Clone, Copy)]
pub enum KeywordType {
    Fun,
    Return,
    Let,
}

#[derive(Debug, Clone, Copy)]
pub enum OpType {
    Add, Sub, Mul, Div, Mod,

    LessThan,
    GreaterThan,
    LessThanEqualTo,
    GreaterThanEqualTo,
    EqualTo,
    NotEqualTo,

    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,

    Comma,
    OpenCurlyBrace,
    CloseCurlyBrace,
    OpenParen,
    CloseParen,
}

const OPERATOR_MAPPING: &[(&str, OpType)] = &[
    ("==", OpType::EqualTo),
    ("!=", OpType::NotEqualTo),
    ("<=", OpType::LessThanEqualTo),
    (">=", OpType::GreaterThanEqualTo),
    ("+=", OpType::AddAssign),
    ("-=", OpType::SubAssign),
    ("/=", OpType::DivAssign),
    ("*=", OpType::MulAssign),
    ("%=", OpType::ModAssign),
    ("<",  OpType::LessThan),
    (">",  OpType::GreaterThan),
    ("+",  OpType::Add),
    ("-",  OpType::Sub),
    ("/",  OpType::Div),
    ("*",  OpType::Mul),
    ("%",  OpType::Mod),
    ("=",  OpType::Assign),
    ("{",  OpType::OpenCurlyBrace),
    ("}",  OpType::CloseCurlyBrace),
    ("(",  OpType::OpenParen),
    (")",  OpType::CloseParen),
    (",",  OpType::Comma),
];

impl KeywordType {
    fn from_str(str: &str) -> Option<Self> {
        use KeywordType::*;

        match str {
            "fun" => Some(Fun),
            "return" => Some(Return),
            "let" => Some(Let),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Lexer {
    contents: Vec<char>,
    next_index: usize,
}

impl Lexer {
    pub fn new(contents: String) -> Self {
        return Self {
            contents: contents.chars().collect(),
            next_index: 0,
        };
    }

    pub fn from_file(path: &str) -> Self {
        let contents = read_to_string(path).expect("Invalid path!");
        return Self::new(contents);
    }

    fn peek_ch(&self) -> Option<char> {
        let len = self.contents.len();
        if !(0..len).contains(&self.next_index) {
            return None;
        }
        return Some(self.contents[self.next_index]);
    }

    fn next_ch(&mut self) -> Option<char> {
        let ch = self.peek_ch()?;
        self.next_index += 1;
        return Some(ch);
    }

    fn capture_while<F>(&mut self, predicate: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut capture = String::new();
        while let Some(ch) = self.peek_ch() {
            if predicate(ch) {
                capture.push(ch);
                self.next_ch();
            } else {
                break;
            }
        }
        return capture;
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek_ch() {
            if !ch.is_ascii_whitespace() {
                return;
            }
            self.next_ch();
        }
    }

    fn capture(&mut self, str: &str) -> bool {
        for (i, ch) in str.chars().enumerate() {
            if i + self.next_index > self.contents.len() {
                return false;
            }
            if self.contents[self.next_index + i] != ch {
                return false;
            }
        }
        self.next_index += str.len();
        return true;
    }

    fn capture_invalid(&mut self, ch: char) -> Token {
        self.next_ch();
        return Token::Invalid(ch);
    }

    fn capture_operator(&mut self) -> Option<OpType> {
        for (string, op_type) in OPERATOR_MAPPING.iter() {
            if self.capture(string) {
                return Some(*op_type);
            }
        }
        return None;
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;

        self.skip_whitespace();
        let ch = self.peek_ch()?;
        let token = match ch {

            ch if !ch.is_alphanumeric() => if let Some(op_type) = self.capture_operator() {
                Op(op_type)
            } else {
                self.capture_invalid(ch)
            }
            ch if ch.is_numeric() => {
                let mut literal = self.capture_while(|c| c.is_numeric());
                if self.capture(".") {
                    literal += ".";
                    literal += &self.capture_while(|c| c.is_numeric());
                }
                Literal(literal)
            }
            ch if ch.is_alphabetic() || ch == '_' => {
                let ident = self.capture_while(|c| c.is_alphanumeric() || ch == '_');

                if let Some(keyword_type) = KeywordType::from_str(&ident) {
                    Keyword(keyword_type)
                } else {
                    Ident(ident)
                }
            }
            _ => {
                self.capture_invalid(ch)
            }
        };
        return Some(token);
    }
}
