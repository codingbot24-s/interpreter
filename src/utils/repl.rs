pub mod repl {
    use std::io::{self, Write};

    use crate::utils::{
        lexer::Lexer,
        token::token::{Token, TokenType},
    };
    const PROMPT: &str = ">> ";

    pub fn start() {
        println!("Welcome to the Chimpanzee Programming Language REPL!");
        println!("Commands:");
        println!("  exit, quit - Exit the REPL");
        println!();
        loop {
            print!("{}", PROMPT);
            io::stdout().flush().unwrap();

            let mut input: String = String::new();
            io::stdin().read_line(&mut input).unwrap();
            input = input.trim().to_string();
            println!("Input is {}", input);
            if input == "exit" {
                break;
            }
            let mut l = Lexer::new(&input);
            loop {
                match l.next_token() {
                    Some(token) => {
                        if token.token == TokenType::EOF {
                           break; 
                        }
                        println!("{:?} -> '{}'", token.token, token.litreal);
                    }
                    None => {
                        println!("Error getting token");
                        break;
                    }  
                }
            } 
        }
        println!()
    }
}
