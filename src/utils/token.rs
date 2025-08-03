
pub enum Token {
    // ILLEGAL signifies a token/character we don’t know about
    ILLEGAL(String),
    // EOF stands for “end of file”, which tells our parser later on that it can stop. 
    EOF(String),
    IDENT(String) ,
    INT(String),
    ASSIGN (String),
    PLUS (String),
    COMMA(String),
    SEMICOLON(String),
    LPAREN(String),
    RPAREN(String),
    LBRACE(String),
    RBRACE(String),
    FUNCTION(String),
    LET(String),
}


