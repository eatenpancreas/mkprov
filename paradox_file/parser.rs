use crate::{Object, ParadoxFile};
use crate::lexer::{Lexer, Token};

pub struct Parser {
    tokens: Vec<Token>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens }
    }
    
    pub fn parse(&self) -> Object {
        Object {
            fields: vec![],
        }
    }
}
