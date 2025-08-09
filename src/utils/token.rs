pub mod token {
    
    #[derive(PartialEq,Clone,Copy,Debug)]
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
        UNDEFINED
    }
    #[derive(Debug)]
    pub struct Token {
        token:TokenType,
        litreal:String
    }

    pub fn new_token (token_type:TokenType,ch:char) -> Token{
        Token {
            token:token_type,
            litreal:ch.to_string(),
        }                
    }
        
}


