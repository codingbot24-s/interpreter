

pub mod ast {
    pub trait Node {
        fn token_literal() -> str;
    }    
    pub trait Statement {
        fn statement_node();   
    }
    pub trait Expression {
        fn expression_node();
    }
    pub struct Program {
        
    }
}