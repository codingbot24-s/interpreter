use super::token::token::{Token, TokenType, new_token};

struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

enum LE {
    UNDEFINEDTOKEN,
}

static TOKEN_TABLE: [TokenType; 256] = {
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
    fn new(&mut self, input: String) -> Self {
        let mut l = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: '0',
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '0';
        } else {
            //TODO: parse the next input that is on the next read_position into the char
            self.ch = self.input.as_bytes()[self.read_position] as char
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Result<Token, LE> {
        let ch_byte = self.ch as u8;
        let token_type = TOKEN_TABLE[ch_byte as usize];

        if token_type == TokenType::UNDEFINED {
            self.read_char();
            Err(LE::UNDEFINEDTOKEN)
        } else {
            let tok = new_token(token_type, self.ch);
            self.read_char();
            Ok(tok)
        }
    }
}
