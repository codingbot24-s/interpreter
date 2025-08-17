
pub mod repl {
    use std::io;

    use crate::utils::{lexer::Lexer, token::token::{Token, TokenType}};
    pub fn start () {
    println!("Hello This is the Chimpanzi programming language!");
    println!("Feel Free to Type in Commands!");
    println!(">>");

    loop {
        let mut input:String = String::new();
        io::stdin().read_line(&mut input);
        input = input.trim().to_string();
        println!("Input is {}",input);
        if input == "exit" {
            break;
        }
        let mut l = Lexer::new(&input); 
        if let Some(mut tok) = l.next_token() {
            while tok.token != TokenType::EOF {
               match l.next_token() {
                Some(next_tok) =>   tok = next_tok,
                None => println!("Error in next token") 
               }
               println!("{:?}",tok)
            }
        }
    } 
    
   }
}
