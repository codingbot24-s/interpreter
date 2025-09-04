pub mod parser {
    use crate::utils::{
        lexer::Lexer,
        token::{token::Token,token::TokenType},
    };

    struct Parser {
        l: Box<Lexer>,
        cur_token: Token,
        peek_token: Token,
    }
    impl Parser {

        pub fn new(lexer: Lexer) -> Parser {
            let mut p = Parser {
                l: Box::new(lexer),
                cur_token: Token {
                    token: TokenType::UNDEFINED,
                    litreal: String::new(),
                },
                peek_token: Token {
                    token: TokenType::UNDEFINED,
                    litreal: String::new(),
                },
            };
            p.next_token(); 
            p
        }

        pub fn next_token(&mut self) {
            // added clone on Token type
            self.cur_token = self.peek_token.clone();
            if let Some(tok) = self.l.next_token() {
                self.peek_token = tok
            }
        }

        pub fn parse_program () {

        } 
    }
}
