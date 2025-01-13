mod lexer;
mod token;

use std::ops::Neg;

pub use lexer::*;
use LexerAction::*;

use crate::{Date, Literal, Precision};

enum LexerAction<'a> {
    NewToken(Token),
    ContinueTokenIfNext(&'a dyn Fn(char) -> bool),
    ContinueNewTokenIfNext(Token, &'a dyn Fn(char) -> bool),
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
                (Some(Token::Comment(s)), c) => {
                    s.push(c);
                    if c == '\n' {
                        break;
                    } else {
                        continue;
                    }
                }
                (Some(Token::Whitespace(s)), c) => {
                    s.push(c);
                    ContinueTokenIfNext(&|c| c.is_ascii_whitespace())
                }
                (Some(Token::Literal(Literal::String(s))), c)
                    if is_valid_ident(c) || c.is_ascii_digit() =>
                {
                    s.push(c);
                    ContinueTokenIfNext(&|c| is_valid_ident(c) || c.is_ascii_digit())
                }
                (Some(Token::ExplicitString(s)), c)
                    if c.is_ascii_whitespace() || c.is_ascii_digit() || is_valid_ident(c) =>
                {
                    s.push(c);
                    continue;
                }
                (None, c) if c.is_ascii_whitespace() => {
                    let token = Token::Whitespace(c.to_string());
                    ContinueNewTokenIfNext(token, &|c| c.is_ascii_whitespace())
                }
                (None, c) if is_valid_ident(c) => {
                    let token = Token::Literal(Literal::String(c.to_string()));
                    ContinueNewTokenIfNext(token, &|c| is_valid_ident(c))
                }
                (None, '#') => NewToken(Token::Comment(String::new())),
                (None, c) if c == '-' => {
                    let n = self.pop();
                    if n.is_some_and(|c| c.is_ascii_digit()) {
                        match self.parse_number(n.unwrap(), false) {
                            Ok(n) => SingleToken(Token::Literal(n)),
                            Err(e) => return e.err(),
                        }
                    } else {
                        return LexerError::UnexpectedToken(c, self.cursor() - 1).err();
                    }
                }
                (None, c) if c.is_ascii_digit() => match self.parse_number(c, true) {
                    Ok(n) => SingleToken(Token::Literal(n)),
                    Err(e) => return e.err(),
                },
                (None, '"') => NewToken(Token::ExplicitString(String::new())),
                (None, '=') => SingleToken(Token::Equals),
                (None, '{') => SingleToken(Token::BracketL),
                (None, '}') => SingleToken(Token::BracketR),
                (_, c) => return LexerError::UnexpectedToken(c, self.cursor()).err(),
            };

            match action {
                NewToken(t) => token = Some(t),
                SingleToken(t) => return Some(Ok(t)),
                ContinueTokenIfNext(do_continue) => {
                    if !self.peek().is_some_and(do_continue) {
                        return Some(Ok(token?));
                    }
                }
                ContinueNewTokenIfNext(t, do_continue) => {
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

impl Lexer {
    fn parse_number(&mut self, c: char, positive: bool) -> Result<Literal, LexerError> {
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
                return Err(LexerError::UnexpectedToken(nc, self.cursor()));
            }
        }

        Ok(match numbers.len() - 1 {
            0 => {
                let mut num: i64 = numbers[0].parse().unwrap();
                if !positive {
                    num = num.neg();
                }
                Literal::I64(num)
            }
            1 => {
                let mut num: f32 = numbers.join(".").parse().unwrap();
                if !positive {
                    num = num.neg();
                }
                Literal::F32(num, Precision::new(numbers[1].len()))
            }
            2 => Literal::Date(Date::parse([&numbers[0], &numbers[1], &numbers[2]])?),
            l => return Err(LexerError::TooManyDots(l)),
        })
    }
}

fn is_valid_ident(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}
