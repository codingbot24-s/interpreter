
struct Lexer {
    input :String,
    position :i32, // index of current character
    read_position:i32, // index of next character
    c:i32 // actual char
}

impl Lexer {
    fn new (&mut self,input:String) -> Self {
        let l = Lexer {
            input,
            position:0,
            read_position:0,
            c:0,
        };

        l
    }
}
