

pub mod ast {
    pub trait Node {
        fn token_literal() -> str;
    }    
    pub trait Statement : Node { 
        fn statement_node();   
    }
    
    pub trait Expression : Node{
        fn expression_node();
    }
    pub struct Program {
        
    }
}