pub mod ast {
    pub trait Node {
        fn token_literal(&self) -> &str;
    }
    pub trait Statement: Node {
        fn statement_node();
    }

    pub trait Expression: Node {
        fn expression_node();
    }
    pub struct Program<T> {
        statments: [T],
    }

    impl<T> Node for Program<T>
    where
        T: Statement,
    {
        fn token_literal(&self) -> &str {
            if self.statments.len() > 0 {
                self.statments[0].token_literal()
            } else {
                return "";
            }
        }
    }
}
