#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Selector(String), // #id .class div
    Comma,            // ,
    LBrace,           // {
    RBrace,           // }
    Backslash,        // \
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Lexer {
        Lexer { input, position: 0 }
    }

    pub fn token(&mut self) -> Option<Token> {
        use std::iter::FromIterator;
        while self.curr().is_some() && self.curr().unwrap().is_whitespace() {
            self.next();
        }

        let curr = self.curr()?;
        let token = match curr {
            &',' => Some(Token::Comma),
            &'{' => Some(Token::LBrace),
            &'}' => Some(Token::RBrace),
            &'\\' => Some(Token::Backslash),
            c => Some(Token::Selector(c.to_string())),
        };
        self.next();
        return token;
    }

    fn tokenSelector(&mut self, curr: &char) -> Option<Token> {
        use std::iter::FromIterator;
        while self.curr().is_some() && self.curr().unwrap().is_whitespace() {
            self.next();
        }

        let token = match curr {
            &',' => Some(Token::Comma),
            &'{' => Some(Token::LBrace),
            &'}' => Some(Token::RBrace),
            &'\\' => Some(Token::Backslash),
            _ => None,
        };
        self.next();
        return token;
    }

    fn next(&mut self) {
        self.position += 1;
    }

    fn curr(&mut self) -> Option<&char> {
        self.input.get(self.position)
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.get(self.position + 1)
    }

    fn is_number(c: &char) -> bool {
        c.is_ascii_digit() || c == &'.'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new(".a { }".chars().collect());
        assert_eq!(lexer.token(), Some(Token::Selector(".".to_string())));
        assert_eq!(lexer.token(), Some(Token::Selector("a".to_string())));
        assert_eq!(lexer.token(), Some(Token::LBrace));
        assert_eq!(lexer.token(), Some(Token::RBrace));
        assert_eq!(lexer.token(), None);
    }
}
