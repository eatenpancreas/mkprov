mod lexer;
mod token;

pub use lexer::*;
use LexerAction::*;

use crate::{Date, Literal};

enum LexerAction<'a> {
    ContinueTokenIf(&'a dyn Fn(char) -> bool),
    ContinueNewTokenIf(Token, &'a dyn Fn(char) -> bool),
    SingleToken(Token),
}

impl Iterator for Lexer {
    type Item = Result<Token, LexerError>;

    /// takes any word that isn't a comment and formats it into separate, consumable chunks
    /// with a location of origin
    fn next(&mut self) -> Option<Self::Item> {
        let mut token = None;

        while let Some(char) = self.pop() {
            let action = match (&mut token, char) {
                (Some(Token::ExplicitString(_)), '"') => break,
                (Some(Token::ExplicitString(_)), '\n') => {
                    return LexerError::UnexpectedEndOfLine(self.cursor()).err()
                }
                (Some(Token::Whitespace(s)), c) if c.is_ascii_whitespace() => {
                    s.push(c);
                    ContinueTokenIf(&|c| c.is_ascii_whitespace())
                }
                (Some(Token::Literal(Literal::String(s))), c) if is_valid_ident(c) => {
                    s.push(c);
                    ContinueTokenIf(&|c| c.is_ascii_alphanumeric())
                }
                (Some(Token::ExplicitString(s)), c)
                    if c.is_ascii_whitespace() || c.is_ascii_digit() || is_valid_ident(c) =>
                {
                    s.push(c);
                    continue;
                }
                (Some(Token::Comment(s)), c) => {
                    s.push(c);
                    ContinueTokenIf(&|c| c != '\n')
                }
                (None, c) if c.is_ascii_whitespace() => {
                    let token = Token::Whitespace(c.to_string());
                    ContinueNewTokenIf(token, &|c| c.is_whitespace())
                }
                (None, c) if c.is_ascii_alphabetic() => {
                    let token = Token::Literal(Literal::String(c.to_string()));
                    ContinueNewTokenIf(token, &|c| is_valid_ident(c))
                }
                (None, c) if c.is_ascii_digit() => {
                    let mut numbers = vec![c.to_string()];

                    while let Some(nc) = self.peek() {
                        let num = numbers.last_mut().unwrap();
                        if nc.is_ascii_digit() {
                            self.increment();
                            num.push(nc);
                        } else if nc == '.' {
                            self.increment();
                            numbers.push(String::new());
                        } else if nc.is_ascii_whitespace() {
                            break;
                        } else {
                            return LexerError::UnexpectedToken(nc, self.cursor()).err();
                        }
                    }

                    let number = (|| {
                        Ok(match numbers.len() - 1 {
                            0 => Literal::U32(numbers[0].parse()?),
                            1 => Literal::F32(numbers.join("").parse().unwrap()),
                            2 => Literal::Date(Date::new(
                                numbers[0].parse()?,
                                numbers[1].parse()?,
                                numbers[2].parse()?,
                            )),
                            l => return Err(LexerError::TooManyDots(l)),
                        })
                    })();

                    match number {
                        Ok(n) => SingleToken(Token::Literal(n)),
                        Err(e) => return e.err(),
                    }
                }
                (None, '"') => SingleToken(Token::ExplicitString(String::new())),
                (None, '#') => SingleToken(Token::Comment(String::new())),
                (None, '=') => SingleToken(Token::Equals),
                (None, '{') => SingleToken(Token::BracketL),
                (None, '}') => SingleToken(Token::BracketR),
                (_, c) => return LexerError::UnexpectedToken(c, self.cursor()).err(),
            };

            match action {
                SingleToken(t) => return Some(Ok(t)),
                ContinueTokenIf(do_continue) => {
                    if !self.peek().is_some_and(do_continue) {
                        return Some(Ok(token?));
                    }
                }
                ContinueNewTokenIf(t, do_continue) => {
                    if self.peek().is_some_and(do_continue) {
                        token = Some(t);
                    } else {
                        return Some(Ok(t));
                    }
                }
            }
        }

        Some(Ok(token?))
    }
}

fn is_valid_ident(c: char) -> bool {
    c.is_ascii_alphabetic() && c == '_'
}
