mod token;
use thiserror::Error;
pub use token::*;

#[derive(Clone)]
pub struct Lexer {
    cursor: usize,
    characters: Vec<char>,
}

#[derive(Error, Debug, Clone, Copy)]
pub enum LexerError {
    #[error("Unexpected end of file at character {0})")]
    UnexpectedEndOfFile(usize),
    #[error("Unexpected end of line at character {0}")]
    UnexpectedEndOfLine(usize),
    #[error("Unexpected '{0}' at character {1}")]
    UnexpectedToken(char, usize),
}

impl LexerError {
    fn err(self) -> Option<Result<Token, Self>> {
        Some(Err(self))
    }
}

impl Lexer {
    pub fn new(string: &str) -> Lexer {
        Lexer {
            cursor: 0,
            characters: string.chars().collect(),
        }
    }

    /// Returns the next character (if available) and advances the cursor.
    fn pop(&mut self) -> Option<char> {
        let item = self.peek();
        self.increment();
        item
    }

    fn peek(&self) -> Option<char> {
        self.characters.get(self.cursor).map(|c| *c)
    }

    fn increment(&mut self) {
        self.cursor += 1;
    }
}

impl Iterator for Lexer {
    type Item = Result<Token, LexerError>;

    /// takes any word that isn't a comment and formats it into separate, consumable chunks
    /// with a location of origin
    fn next(&mut self) -> Option<Self::Item> {
        let mut token = None;

        while let Some(char) = self.pop() {
            let token_type = token.as_deref_mut();
            token = Some(Token::new(
                self.cursor,
                match (token_type, char) {
                    (Some(TokenType::ExplicitString(_)), '"') => break,
                    (Some(TokenType::ExplicitString(_)), '\n') => {
                        return LexerError::UnexpectedEndOfLine(self.cursor).err()
                    }
                    (Some(TokenType::ExplicitString(s)), c) => {
                        s.push(c);
                        continue;
                    }
                    (Some(TokenType::Comment(comment)), c) => {
                        comment.push(c);
                        if let Some('\n') = self.peek() {
                            break;
                        } else {
                            continue;
                        }
                    }
                    (None, '"') => TokenType::ExplicitString(String::new()),
                    (None, '#') => TokenType::Comment(String::new()),
                    (None, '\n') => TokenType::Newline,
                    (None, '=') => TokenType::Equals,
                    (None, '{') => TokenType::BracketL,
                    (None, '}') => TokenType::BracketR,
                },
            ));

            // Literal strings defined by " marks
            // if *char == '"' && !is_comment {
            //     is_string = !is_string;
            //     if is_string {
            //         is_lit_stringed = true
            //     };
            // } else if *char == '=' && !is_string && !is_comment {
            //     content.push(*char);
            //     location = self.cursor - 1;
            //     return Some(Ok(Token::parse(
            //         location,
            //         content.as_str(),
            //         is_lit_stringed,
            //     )));
            // } else if *char == '#' && !is_string {
            //     is_comment = true
            // }
            // // Enable comment mode
            // else if is_comment {
            //     // End comments on newlines
            //     if *char == '\n' {
            //         is_comment = false;
            //         // When encountering whitespace as token already contains something, return
            //         if started {
            //             return Some(Ok(Token::parse(
            //                 location,
            //                 content.as_str(),
            //                 is_lit_stringed,
            //             )));
            //         }
            //     }
            // } else if !char.is_whitespace() || is_string {
            //     content.push(*char);

            //     if !started {
            //         location = self.cursor - 1;
            //         started = true;
            //     }
            // } else if started {
            //     // When encountering whitespace as token already contains something, return
            //     return Some(Ok(Token::parse(
            //         location,
            //         content.as_str(),
            //         is_lit_stringed,
            //     )));
            // }
        }

        Some(Ok(token?))
    }
}

// pub fn parse(, is_lit_stringed: bool) -> Option<TokenType> {
//     if is_lit_stringed {
//         Some(TokenType::Literal(Literal::String(content)))
//     } else if content.starts_with(|ch: char| ch.is_numeric()) {
//         Literal::parse_numeric(&content).and_then(|n| Some(TokenType::Literal(n)))
//         // special characters
//     } else if content == "=" {
//         Some(TokenType::Equals)
//     } else if content == "{" {
//         Some(TokenType::BracketL)
//     } else if content == "}" {
//         Some(TokenType::BracketR)
//     } else {
//         // regular string
//         Some(TokenType::Literal(Literal::String(content)))
//     }
// }
