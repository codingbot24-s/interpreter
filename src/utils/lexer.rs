
struct Lexer {
    input :String,
    current_position :usize, // index of current character
    next_position :usize, // index of next character
    current_char :u8, // current character
}

impl Lexer {
    fn new (&mut self,input:String) -> Self {
       let mut l = Lexer { 
            input: input, 
            current_position: 0, 
            next_position: 0, 
            current_char: 0 
        }; 
        l.read_char();
        l
    }

    fn read_char (&mut self) {
        // check end of input
        if self.next_position >= self.input.len() {
            self.current_char = 0
        }
        else {
            self.current_char = self.input.as_bytes()[self.next_position];   
        }

        self.current_position = self.next_position;
        self.next_position +=1;
    }
    
}


