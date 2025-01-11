use crate::Token;
use thiserror::Error;

pub struct Lexer {
    cursor: usize,
    characters: Vec<char>,
}

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Parsing error: String did not end until file")]
    StringNotEnded,
}

impl Lexer {
    pub fn new(string: &str) -> Self {
        Self {
            characters: string.chars().collect(),
            cursor: 0,
        }
    }

    pub fn unwrap_all(self) -> Vec<Token> {
        self.filter_map(|res| match res {
            Ok(token) => Some(token),
            Err(err) => {
                eprintln!("{err}");
                None
            }
        })
        .collect()
    }

    /// Returns the next character (if available) and advances the cursor.
    fn pop(&mut self) -> Option<&char> {
        match self.characters.get(self.cursor) {
            Some(character) => {
                self.cursor += 1;

                Some(character)
            }
            None => None,
        }
    }

    fn peek(&self) -> Option<&char> {
        self.characters.get(self.cursor)
    }
}

impl Iterator for Lexer {
    type Item = Result<Token, LexerError>;

    /// takes any word that isn't a comment and formats it into separate, consumable chunks
    /// with a location of origin
    fn next(&mut self) -> Option<Self::Item> {
        let mut started = false;
        let mut location = 0;
        let mut content = String::new();
        let mut is_lit_stringed = false;
        let mut is_comment = false;
        let mut is_string = false;

        if self.peek().is_some_and(|c| *c == '}') && !is_lit_stringed && content.len() > 0 {
            return Some(Ok(Token::new(location, content.as_str(), is_lit_stringed)));
        }

        while let Some(char) = self.pop() {
            // Literal strings defined by " marks
            if *char == '"' && !is_comment {
                is_string = !is_string;
                if is_string {
                    is_lit_stringed = true
                };
            } else if *char == '=' && !is_string && !is_comment {
                content.push(*char);
                location = self.cursor - 1;
                return Some(Ok(Token::new(location, content.as_str(), is_lit_stringed)));
            } else if *char == '#' && !is_string {
                is_comment = true
            }
            // Enable comment mode
            else if is_comment {
                // End comments on newlines
                if *char == '\n' {
                    is_comment = false;
                    // When encountering whitespace as token already contains something, return
                    if started {
                        return Some(Ok(Token::new(location, content.as_str(), is_lit_stringed)));
                    }
                }
            } else if !char.is_whitespace() || is_string {
                content.push(*char);

                if !started {
                    location = self.cursor - 1;
                    started = true;
                }
            } else if started {
                // When encountering whitespace as token already contains something, return
                return Some(Ok(Token::new(location, content.as_str(), is_lit_stringed)));
            }
        }

        if is_string {
            Some(Err(LexerError::StringNotEnded))
        } else {
            None
        }
    }
}
