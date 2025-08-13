use super::token::token::{Token, TokenType, lookup_ident, new_token};

pub struct Lexer {
    input: String,
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
    table[b'0' as usize] = TokenType::EOF;
    table
};

impl Lexer {
    pub fn new(input: String) -> Self {
        let l = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: '0',
        };
        l
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '0';
        } else {
            self.ch = self.input.as_bytes()[self.read_position] as char;
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let ch_byte = self.ch as u8;
        let token_type = TOKEN_TABLE[ch_byte as usize];

        if token_type != TokenType::UNDEFINED {
            let token = new_token(token_type, self.ch);
            self.read_char();
            Some(token)
        } else if is_letter(&self.ch) {
            let literal = self.read_identifier();
            let tok_type = lookup_ident(literal);
            let tok: Token = Token {
                litreal: literal.to_string(),
                token: *tok_type,
            };
            Some(tok)
        } else if is_digit(&self.ch) {
            let tok_type = TokenType::INT;
            // TODO: Bug while loop is infinite if inputs is single INT
            let literal = self.read_num();
            println!("After read num litreal is {}",literal );
            let tok = Token {
                litreal: literal.to_string(),
                token: tok_type,
            };
            Some(tok)
        } else {
            self.read_char();
            let token = new_token(TokenType::ILLEGAL, self.ch);
            return Some(token);
        }
    }
    // it reads in an identifier and advances our lexer’s positions until it encounters a non-letter-character.
    pub fn read_identifier(&mut self) -> &str {
        let position = self.position;

        while is_letter(&self.ch) {
            self.read_char();
        }
        let str = &self.input[position..self.read_position - 1];
        return str;
    }
    pub fn read_num(&mut self) -> &str {
        let position = self.position;
        while is_digit(&self.ch) {
            self.read_char();
        }
        
        return &self.input[position..self.position];
    }
}

pub fn is_letter(ch: &char) -> bool {
    // TODO: we can implement fn for skiping white space
    ch.is_alphabetic() || *ch as u8 == b'_' || ch.is_whitespace()
}

pub fn is_digit(ch: &char) -> bool {
    ch.is_numeric()
}
