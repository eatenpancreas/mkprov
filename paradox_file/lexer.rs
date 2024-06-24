use crate::Location;

pub(crate) struct Lexer {
    cursor: usize,
    characters: Vec<char>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Token {
    pub(crate) location: Location,
    pub(crate) content: String
}

impl Token {
    pub fn is(&self, string: &str) -> bool {
        self.content == string
    }
}

impl Lexer {
    pub fn new(string: &str) -> Self {
        Self {
            characters: string.chars().collect(),
            cursor: 0,
        }
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
}

impl Iterator for Lexer {
    type Item = Token;

    /// takes any word that isn't a comment and formats it into separate, consumable chunks
    /// with a location of origin
    fn next(&mut self) -> Option<Self::Item> {
        let mut started = false;
        let mut ident = Token {
            location: Location(0),
            content: String::new(),
        };
        let mut is_comment = false;
        let mut is_string = false;

        while let Some(char) = self.pop() {
            if *char == '"' && !is_comment { is_string = !is_string }
            else if *char == '#' && !is_string { is_comment = true }
            else if is_comment {
                if *char == '\n' { is_comment = false }
            } else if !char.is_whitespace() || is_string {
                ident.content.push(*char);

                if !started {
                    ident.location.0 = self.cursor - 1;
                    started = true;
                }
            } else if started {
                return Some(ident)
            }
        }

        if is_string {
            eprintln!("Parsing error: String did not end until end of file");
        }

        None
    }
}