use std::string::String;
use std::{iter::Peekable, str::Chars};

use crate::error::{Error, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Ident(String),
    String(String),
    Number(String),
    OpenParen,
    CloseParen,
    Comma,
    Semicolon,
    Asterisk,
    Plus,
    Minus,
    Slash,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Create,
    Table,
    Int,
    Integer,
    Boolean,
    Bool,
    String,
    Text,
    Varchar,
    Float,
    Double,
    Select,
    From,
    Insert,
    Into,
    Values,
    True,
    False,
    Default,
    Not,
    Null,
    Primary,
    Key,
}

impl Keyword {
    pub fn from_str(ident: &str) -> Option<Self> {
        return match ident.to_uppercase().as_ref() {
            "CREATE" => Some(Keyword::Create),
            "TABLE" => Some(Keyword::Table),
            "INT" => Some(Keyword::Int),
            "INTEGER" => Some(Keyword::Integer),
            "BOOLEAN" => Some(Keyword::Boolean),
            "BOOL" => Some(Keyword::Bool),
            "STRING" => Some(Keyword::String),
            "TEXT" => Some(Keyword::Text),
            "VARCHAR" => Some(Keyword::Varchar),
            "FLOAT" => Some(Keyword::Float),
            "DOUBLE" => Some(Keyword::Double),
            "SELECT" => Some(Keyword::Select),
            "FROM" => Some(Keyword::From),
            "INSERT" => Some(Keyword::Insert),
            "INTO" => Some(Keyword::Into),
            "VALUES" => Some(Keyword::Values),
            "TRUE" => Some(Keyword::True),
            "FALSE" => Some(Keyword::False),
            "DEFAULT" => Some(Keyword::Default),
            "NOT" => Some(Keyword::Not),
            "NULL" => Some(Keyword::Null),
            "PRIMARY" => Some(Keyword::Primary),
            "KEY" => Some(Keyword::Key),
            _ => None,
        };
    }
}

pub struct Lexer<'a> {
    iter: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.scan() {
            Ok(Some(token)) => Some(Ok(token)),
            Ok(None) => self
                .iter
                .peek()
                .map(|c| Err(Error::Parse(format!("[Lexer] Unexpeted character {}", c)))),
            Err(err) => Some(Err(err)),
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(sql_text: &'a str) -> Self {
        return Self {
            iter: sql_text.chars().peekable(),
        };
    }

    fn erase_whitespace(&mut self) {
        self.next_while(|c| c.is_whitespace());
    }

    fn next_while<F: Fn(char) -> bool>(&mut self, predicate: F) -> Option<String> {
        let mut value = std::string::String::new();
        while let Some(c) = self.next_if(&predicate) {
            value.push(c);
        }
        Some(value).filter(|v| !v.is_empty())
    }

    fn next_if<F: Fn(char) -> bool>(&mut self, predicate: F) -> Option<char> {
        self.iter.peek().filter(|&c| predicate(*c))?;
        return self.iter.next();
    }

    fn next_if_token<F: Fn(char) -> Option<Token>>(&mut self, predicate: F) -> Option<Token> {
        let token = self.iter.peek().and_then(|c| predicate(*c))?;
        self.iter.next();
        return Some(token);
    }

    /// 扫描单引号字符串，处理转义的单引号（两个连续的单引号）
    fn scan_string(&mut self) -> Result<Option<Token>> {
        if self.next_if(|c| c == '\'').is_none() {
            return Ok(None);
        }

        let mut val = String::new();
        loop {
            match self.iter.next() {
                Some('\'') => break,
                Some(c) => val.push(c),
                None => return Err(Error::Parse(format!("[Lexer] Unexpected end of string"))),
            }
        }

        Ok(Some(Token::String(val)))
    }

    /// 扫描数字，支持整数和浮点数
    fn scan_number(&mut self) -> Option<Token> {
        let mut num = self.next_while(|c| c.is_ascii_digit())?;
        if let Some(sep) = self.next_if(|c| c == '.') {
            num.push(sep);
            while let Some(c) = self.next_if(|c| c.is_ascii_digit()) {
                num.push(c);
            }
        }
        return Some(Token::Number(num));
    }

    /// 扫描符号：* ( ) , ; + - /
    fn scan_symbol(&mut self) -> Option<Token> {
        return self.next_if_token(|c| match c {
            '*' => return Some(Token::Asterisk),
            '(' => return Some(Token::OpenParen),
            ')' => return Some(Token::CloseParen),
            ',' => return Some(Token::Comma),
            ';' => return Some(Token::Semicolon),
            '+' => return Some(Token::Plus),
            '-' => return Some(Token::Minus),
            '/' => return Some(Token::Slash),
            _ => return None,
        });
    }

    /// 扫描标识符，先匹配首字母，后续字符可以是字母、数字或下划线
    /// 如果标识符匹配到关键字，则返回 Keyword Token，否则返回 Ident Token
    fn scan_ident(&mut self) -> Option<Token> {
        let mut value = self.next_if(|c| c.is_alphabetic())?.to_string();
        while let Some(c) = self.next_if(|c| c.is_alphanumeric() || c == '_') {
            value.push(c);
        }

        return Some(
            Keyword::from_str(&value).map_or(Token::Ident(value.to_lowercase()), Token::Keyword),
        );
    }

    /// 根据首字符类型分发到不同的扫描函数
    fn scan(&mut self) -> Result<Option<Token>> {
        self.erase_whitespace();
        match self.iter.peek() {
            Some('\'') => self.scan_string(),
            Some(c) if c.is_ascii_digit() => Ok(self.scan_number()),
            Some(c) if c.is_alphabetic() => Ok(self.scan_ident()),
            Some(_) => Ok(self.scan_symbol()),
            None => Ok(None),
        }
    }
}
