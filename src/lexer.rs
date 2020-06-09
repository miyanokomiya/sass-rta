#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Value(String),   // selector property value
    Comment(String), // // comment /* comment */
    Comma,           // ,
    LBrace,          // {
    RBrace,          // }
    Colon,           // :
    Semicolon,       // ;
                     // Backslash,        // \ TODO: escaping
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
        while self.curr().is_some() && self.curr().unwrap().is_whitespace() {
            self.next();
        }

        let token = if self.curr()? == &'/' && self.peek()? == &'/' {
            self.token_line_comment()
        } else if self.curr()? == &'/' && self.peek()? == &'*' {
            self.token_block_comment()
        } else {
            match self.curr()? {
                &',' => Some(Token::Comma),
                &'{' => Some(Token::LBrace),
                &'}' => Some(Token::RBrace),
                &':' => Some(Token::Colon),
                &';' => Some(Token::Semicolon),
                _ => self.token_value(),
            }
        };

        self.next();
        return token;
    }

    fn token_line_comment(&mut self) -> Option<Token> {
        let mut line_comment = self.curr()?.to_string();
        while self.peek().is_some() && self.peek()? != &'\n' {
            self.next();
            line_comment = line_comment + &self.curr()?.to_string();
        }
        return Some(Token::Comment(line_comment));
    }

    fn token_block_comment(&mut self) -> Option<Token> {
        let mut line_comment = self.curr()?.to_string();
        while self.peek().is_some() {
            self.next();
            line_comment = line_comment + &self.curr()?.to_string();

            if self.peek()? == &'*' && self.peek_peek().is_some() && self.peek_peek()? == &'/' {
                line_comment = line_comment + "*/";
                self.next();
                self.next();
                break;
            }
        }
        return Some(Token::Comment(line_comment));
    }

    fn token_value(&mut self) -> Option<Token> {
        let mut selector = self.curr()?.to_string();
        while self.peek().is_some() && Self::is_selector(self.peek().unwrap()) {
            self.next();
            selector = selector + &self.curr()?.to_string();
        }
        return Some(Token::Value(selector));
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

    fn peek_peek(&mut self) -> Option<&char> {
        self.input.get(self.position + 2)
    }

    fn is_selector(c: &char) -> bool {
        return match c {
            &',' => false,
            &'{' => false,
            &' ' => false,
            &'\t' => false,
            _ => true,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_1() {
        let mut lexer = Lexer::new(".a { }".chars().collect());
        assert_eq!(lexer.token(), Some(Token::Value(".a".to_string())));
        assert_eq!(lexer.token(), Some(Token::LBrace));
        assert_eq!(lexer.token(), Some(Token::RBrace));
        assert_eq!(lexer.token(), None);
    }

    #[test]
    fn test_lexer_2() {
        let mut lexer = Lexer::new(".aa, .bb { }".chars().collect());
        assert_eq!(lexer.token(), Some(Token::Value(".aa".to_string())));
        assert_eq!(lexer.token(), Some(Token::Comma));
        assert_eq!(lexer.token(), Some(Token::Value(".bb".to_string())));
        assert_eq!(lexer.token(), Some(Token::LBrace));
    }

    #[test]
    fn test_lexer_3() {
        let mut lexer = Lexer::new(".a,\n.b { }".chars().collect());
        assert_eq!(lexer.token(), Some(Token::Value(".a".to_string())));
        assert_eq!(lexer.token(), Some(Token::Comma));
        assert_eq!(lexer.token(), Some(Token::Value(".b".to_string())));
        assert_eq!(lexer.token(), Some(Token::LBrace));
    }

    #[test]
    fn test_lexer_4() {
        let mut lexer = Lexer::new(".a .b { }".chars().collect());
        assert_eq!(lexer.token(), Some(Token::Value(".a".to_string())));
        assert_eq!(lexer.token(), Some(Token::Value(".b".to_string())));
        assert_eq!(lexer.token(), Some(Token::LBrace));
    }

    #[test]
    fn test_lexer_line_comment() {
        let mut lexer = Lexer::new(".a // abc \n".chars().collect());
        assert_eq!(lexer.token(), Some(Token::Value(".a".to_string())));
        assert_eq!(lexer.token(), Some(Token::Comment("// abc ".to_string())));
        assert_eq!(lexer.token(), None);
    }

    #[test]
    fn test_lexer_block_comment_1() {
        let mut lexer = Lexer::new(".a /* abc */ {".chars().collect());
        assert_eq!(lexer.token(), Some(Token::Value(".a".to_string())));
        assert_eq!(lexer.token(), Some(Token::Comment("/* abc */".to_string())));
        assert_eq!(lexer.token(), Some(Token::LBrace));
        assert_eq!(lexer.token(), None);
    }

    #[test]
    fn test_lexer_block_comment_2() {
        let mut lexer = Lexer::new(".a /*\n abc \n*/ {".chars().collect());
        assert_eq!(lexer.token(), Some(Token::Value(".a".to_string())));
        assert_eq!(
            lexer.token(),
            Some(Token::Comment("/*\n abc \n*/".to_string()))
        );
        assert_eq!(lexer.token(), Some(Token::LBrace));
        assert_eq!(lexer.token(), None);
    }
}
