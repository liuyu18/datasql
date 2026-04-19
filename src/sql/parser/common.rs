use crate::error::{Error, Result};
use crate::sql::parser::lexer::Lexer;
use crate::sql::parser::lexer::Token;
use std::iter::Peekable;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser {
            lexer: Lexer::new(input).peekable(),
        }
    }
}

pub trait ParserExt {
    fn peek(&mut self) -> Result<Option<Token>>;
    fn next(&mut self) -> Result<Token>;
    fn next_ident(&mut self) -> Result<String>;
    fn next_expect(&mut self, expect: Token) -> Result<()>;
    fn next_if<F: Fn(&Token) -> bool>(&mut self, predicate: F) -> Option<Token>;
    fn next_if_keyword(&mut self) -> Option<Token>;
    fn next_if_token(&mut self, token: Token) -> Option<Token>;
}

impl<'a> ParserExt for Parser<'a> {
    fn peek(&mut self) -> Result<Option<Token>> {
        self.lexer.peek().cloned().transpose()
    }

    fn next(&mut self) -> Result<Token> {
        match self.lexer.next() {
            Some(Ok(token)) => Ok(token),
            Some(Err(e)) => Err(e),
            None => Err(Error::Parse("[Parser] Unexpected end of input".to_string())),
        }
    }

    fn next_ident(&mut self) -> Result<String> {
        match self.next()? {
            Token::Ident(ident) => Ok(ident),
            token => Err(Error::Parse(format!(
                "[Parser] Expected ident, got token {:?}",
                token
            ))),
        }
    }

    fn next_expect(&mut self, expect: Token) -> Result<()> {
        let token = self.next()?;
        if token != expect {
            return Err(Error::Parse(format!(
                "[Parser] Expected token {:?}, got {:?}",
                expect, token
            )));
        }
        Ok(())
    }

    fn next_if<F: Fn(&Token) -> bool>(&mut self, predicate: F) -> Option<Token> {
        self.peek().ok().flatten().filter(|t| predicate(t))?;
        self.next().ok()
    }

    fn next_if_keyword(&mut self) -> Option<Token> {
        self.next_if(|t| matches!(t, Token::Keyword(_)))
    }

    fn next_if_token(&mut self, token: Token) -> Option<Token> {
        self.next_if(|t| t == &token)
    }
}
