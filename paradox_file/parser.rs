use thiserror::Error;

use crate::{Field, KeyVal, Lexer, LexerError, Literal, Location, Object, Token, TokenType};

pub struct Parser {
    token_idx: usize,
    tokens: Vec<Option<Token>>,
    file_length: usize
}

#[derive(Error, Debug, PartialEq)]
pub enum ParserError {
    #[error("{0}: Literal {1} expects another literal, }} or =")]
    LiteralExpectsFriend(Location, Literal),
    #[error("{0}: Unexpected token '{1}'")]
    UnexpectedToken(Location, TokenType),
    #[error("{0}: Unexpected error occurred")]
    UnexpectedError(Location),
}

impl Parser {
    pub fn include_lexer(string: &str) -> Result<Parser, Vec<LexerError>> {
        let length = string.len();
        
        let (oks, errs) = Lexer::new(string)
          .fold((vec![], vec![]), | 
              (mut oks, mut errs), res
          | {
            match res { 
                Ok(ok) => oks.push(Some(ok)), 
                Err(err) => errs.push(err)
            };
            (oks, errs)
        });
        
        if errs.len() > 0 { return Err(errs) }
        
        Ok(Parser::new(oks, length))
    }
    
    fn new(tokens: Vec<Option<Token>>, file_length: usize) -> Parser {
        Parser {
            tokens, 
            token_idx: 0,
            file_length,
        }
    }

    /// Consumes the next character (if available) and advances the cursor.
    fn pop(&mut self) -> Option<Token> {
        let popped =self.tokens.get_mut(self.token_idx)
          .and_then(|x| x.take());
        self.token_idx += 1;
        popped
    }
    
    // views a token and doesn't consume it.
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.token_idx).and_then(|x| x.as_ref())
    }

    pub fn parse(&mut self) -> Result<Object, ParserError> {
        let mut base_fields = vec![];
        let mut closing_location = 0;
        self.parse_fields(&mut base_fields, &mut closing_location, 0)?;
        Ok(Object::new(base_fields, 0))
    }
    
    fn parse_fields(
        &mut self, fields: &mut Vec<Field>, closing_loc: &mut usize, nesting: usize
    ) -> Result<(), ParserError> {

        while let Some(token) = self.pop() { match token.token_type {
            Some(TokenType::Literal(lit)) => {
                let peek = self.peek();
                if peek.is_some_and(|other| other.is_equals()) {
                    self.token_idx += 1;
                    if let Some(val) = self.pop() {
                        // value is an object
                        if val.is_bracket_l() {
                            let mut child_fields = vec![];
                            let mut closing_location = 0;
                            self.parse_fields(&mut child_fields, &mut closing_location,nesting + 1)?;
                            fields.push(Field::new(lit, Object::new(
                                child_fields,
                                nesting + 1
                            )));
                            // value is a regular literal
                        } else if let Some(TokenType::Literal(lit_val)) = val.token_type {
                            fields.push(Field::new(lit, KeyVal::new(lit_val)));
                        }
                    }
                } else if peek.is_some_and(|other| other.is_literal() || other.is_bracket_r()) {
                    fields.push(Field::new_literal(lit));
                } else {
                    return Err(ParserError::LiteralExpectsFriend(token.location, lit));
                }
            }
            Some(TokenType::BracketR) => {
                *closing_loc = token.location.0;
                return Ok(())
            },
            Some(lit) => return Err(ParserError::UnexpectedToken(token.location, lit)),
            _ => {}
        }}

        *closing_loc = self.file_length;
        Ok(())
    }
}
