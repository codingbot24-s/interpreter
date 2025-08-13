use super::token::token::{Token, TokenType, lookup_ident, new_token};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}
pub static TOKEN_TABLE: [TokenType; 256] = {
    let mut table = [const { TokenType::UNDEFINED }; 256];
    table[b'=' as usize] = TokenType::ASSIGN;
    table[b';' as usize] = TokenType::SEMICOLON;
    table[b'(' as usize] = TokenType::LPAREN;
    table[b')' as usize] = TokenType::RPAREN;
    table[b',' as usize] = TokenType::COMMA;
    table[b'+' as usize] = TokenType::PLUS;
    table[b'{' as usize] = TokenType::LBRACE;
    table[b'}' as usize] = TokenType::RBRACE;
    table[b'-' as usize] = TokenType::MINUS;
    table[b'!' as usize] = TokenType::BANG;
    table[b'*' as usize] = TokenType::ASTERISK;
    table[b'/' as usize] = TokenType::SLASH;
    table[b'<' as usize] = TokenType::LT;
    table[b'>' as usize] = TokenType::GT;
table
};

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            // changed
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        l
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }
    pub fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    self.read_char();
                    let tok = Token {
                        token: TokenType::EQ,
                        litreal: "==".to_string(),
                    };
                    return Some(tok);
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    self.read_char();
                    let tok = Token {
                        token: TokenType::NOTEQ,
                        litreal: "!=".to_string(),
                    };
                    return Some(tok);
                }
            }

            _ => {}
        }

        let ch_byte = self.ch as u8;
        let token_type = TOKEN_TABLE[ch_byte as usize];

        if token_type != TokenType::UNDEFINED {
            let token = new_token(token_type, self.ch);
            self.read_char();
            Some(token)
        } else if is_letter(&self.ch) {
            let literal = self.read_identifier();
            let tok_type = lookup_ident(&literal);
            let tok: Token = Token {
                litreal: literal.to_string(),
                token: *tok_type,
            };
            Some(tok)
        } else if is_digit(&self.ch) {
            let tok_type = TokenType::INT;

            let literal = self.read_num();
            println!("After read num litreal is {}", literal);
            let tok = Token {
                litreal: literal.to_string(),
                token: tok_type,
            };
            Some(tok)
        } else if self.ch == '\0' {
            let tok = Token{token:TokenType::EOF,litreal:String::new()};
            Some(tok)
        } else {
            self.read_char();
            let token = new_token(TokenType::UNDEFINED, self.ch);
            return Some(token);
        }
    }
    // it reads in an identifier and advances our lexer’s positions until it encounters a non-letter-character.
    pub fn read_identifier(&mut self) -> String {
        let position = self.position;

        while is_letter(&self.ch) {
            println!("  Reading: '{}' at pos {}", self.ch, self.position);
            self.read_char();
        }

        let result: String = self.input[position..self.position].iter().collect();
        println!("  Result: '{}'", result);
        result
    }
    pub fn read_num(&mut self) -> String {
        let position = self.position;
        while is_digit(&self.ch) {
            self.read_char();
        }

        self.input[position..self.position].iter().collect()
    }
    pub fn peek_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        }
        self.input[self.read_position]
    }
}

pub fn is_letter(ch: &char) -> bool {
    ch.is_alphabetic() || *ch as u8 == b'_'
}

pub fn is_digit(ch: &char) -> bool {
    ch.is_numeric()
}

