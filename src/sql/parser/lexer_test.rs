#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{
        error::Result,
        sql::parser::lexer::{Keyword, Token},
    };
    use crate::sql::parser::lexer::Lexer;

    #[test]
    fn test_lexer_create_table() -> Result<()> {
        let tokens1 = Lexer::new(
            "CREATE table tbl
                (
                    id1 int primary key,
                    id2 integer
                );
                ",
        )
            .peekable()
            .collect::<Result<Vec<_>>>()?;
        eprintln!("tokens1: {:?}", tokens1);
        assert_eq!(
            tokens1,
            vec![
                Token::Keyword(Keyword::Create),
                Token::Keyword(Keyword::Table),
                Token::Ident("tbl".to_string()),
                Token::OpenParen,
                Token::Ident("id1".to_string()),
                Token::Keyword(Keyword::Int),
                Token::Keyword(Keyword::Primary),
                Token::Keyword(Keyword::Key),
                Token::Comma,
                Token::Ident("id2".to_string()),
                Token::Keyword(Keyword::Integer),
                Token::CloseParen,
                Token::Semicolon
            ]
        );
        return Ok(());
    }
}