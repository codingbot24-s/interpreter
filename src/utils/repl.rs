
pub mod repl {
    use std::io;

    use crate::utils::lexer::Lexer;
    pub fn start () {
    println!("Hello This is the Chimpanzi programming language!");
    println!("Feel Free to Type in Commands!");
    println!(">>");
    let mut input:String = String::new();
    loop {
        io::stdin().read_line(&mut input);
        input = input.trim().to_string();
        println!("Input is {}",input);
        if input == "exit" {
            break;
        }
        // creating a lexer will result in borrowing error;     
    } 
    
   }
}
