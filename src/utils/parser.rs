

pub mod parser {
    use crate::utils::{lexer::Lexer,token::{self, token::Token}};

    struct Parser {
        l: Box<Lexer>,
        cur_token: Token,
        peek_token: Token
    } 
    impl Parser {
        pub fn new (lexer:Lexer,) {
            
        }

        pub fn next_token (mut self) {
            self.cur_token = self.peek_token;
           if let Some(tok) = self.l.next_token() {
                self.peek_token = tok
           } 
        }

        pub fn ParseProgram () {

        }
    }

}