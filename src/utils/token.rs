use crate::utils::token::token::TokenType;
use std::collections::HashMap;
use std::sync::LazyLock;

static KEYWORDS: LazyLock<HashMap<&str, TokenType>> =
    LazyLock::new(|| HashMap::from([
        ("fn", TokenType::FUNCTION), 
        ("let", TokenType::LET),
        ("true", TokenType::TRUE),
        ("false", TokenType::FALSE),
        ("if", TokenType::IF),
        ("else", TokenType::ELSE),
        ("return", TokenType::RETURN)
    ]));
pub mod token {
    use crate::utils::token::KEYWORDS;

    #[derive(PartialEq, Clone, Copy, Debug)]
    pub enum TokenType {
        // ILLEGAL signifies a token/character we don’t know about
        ILLEGAL,
        // EOF stands for “end of file”, which tells our parser later on that it can stop.
        EOF,
        IDENT,
        INT,
        ASSIGN,
        PLUS,
        COMMA,
        SEMICOLON,
        LPAREN,
        RPAREN,
        LBRACE,
        RBRACE,
        FUNCTION,
        LET,
        UNDEFINED,
        MINUS,
        BANG,
        ASTERISK,
        SLASH,
        LT,
        GT,
        TRUE,
        FALSE,
        IF,
        ELSE,
        RETURN 
    }
    #[derive(Debug)]
    pub struct Token {
        pub token: TokenType,
        pub litreal: String,
    }

    pub fn new_token(token_type: TokenType, ch: char) -> Token {
        Token {
            token: token_type,
            litreal: ch.to_string(),
        }
    }
    pub fn lookup_ident(ident: &str) -> &TokenType {
        if let Some(tok) = KEYWORDS.get(ident) {
            tok
        } else {
            &TokenType::IDENT
        }
    }
}
