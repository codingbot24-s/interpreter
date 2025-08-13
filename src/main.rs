mod utils;

use utils::lexer::Lexer;

fn main() {
    let input = String::from("let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
".to_string());

    
    let mut l = Lexer::new(input);
    
    let r = l.next_token();   
    match r {
        Some(t) => println!("Found some token {:?}",t),
        _ => println!("Found nothing"),  
    }
}
