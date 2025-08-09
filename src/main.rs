mod utils;

use utils::lexer::Lexer;
fn main() {
    let input = String::from("foo_bar");
    let mut l = Lexer::new(input);
    l.read_char();
    let r = l.next_token();   
    match r {
        Some(t) => println!("Found some token {:?}",t),
        _ => println!("Found nothing"),  
    }
}
