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

#[derive(Debug, PartialEq, Clone)]
pub struct PToken {
    token: Token,
    from: Cursor,
    to: Cursor,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Cursor {
    row: usize,
    column: usize,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    row: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Lexer {
        Lexer {
            input,
            position: 0,
            row: 0,
            column: 0,
        }
    }

    pub fn token(&mut self) -> Option<PToken> {
        {
            self.skip_whitespace();
        }

        let from = self.curr_cursor();

        let token = if self.curr()? == &'/' && self.peek()? == &'/' {
            self.token_line_comment()?
        } else if self.curr()? == &'/' && self.peek()? == &'*' {
            self.token_block_comment()?
        } else {
            match self.curr()? {
                &',' => Token::Comma,
                &'{' => Token::LBrace,
                &'}' => Token::RBrace,
                &':' => Token::Colon,
                &';' => Token::Semicolon,
                _ => self.token_value()?,
            }
        };

        let to = self.curr_cursor();
        self.next();
        return Some(PToken { token, from, to });
    }

    fn skip_whitespace(&mut self) {
        while self.curr().is_some() && self.curr().unwrap().is_whitespace() {
            self.next();
        }
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
        while self.peek().is_some() && Self::is_value(self.peek().unwrap()) {
            self.next();
            selector = selector + &self.curr()?.to_string();
        }
        return Some(Token::Value(selector));
    }

    fn next(&mut self) {
        if self.curr() == Some(&'\n') {
            self.column = 0;
            self.row += 1;
        } else {
            self.column += 1;
        }
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

    fn curr_cursor(&mut self) -> Cursor {
        Cursor {
            row: self.row,
            column: self.column,
        }
    }

    fn is_value(c: &char) -> bool {
        return match c {
            &':' => false,
            &';' => false,
            &',' => false,
            &'{' => false,
            &' ' => false,
            &'\t' => false,
            &'\n' => false,
            _ => true,
        };
    }
}

#[cfg(test)]
mod selector {
    use super::*;

    #[test]
    fn single() {
        let mut lexer = Lexer::new(".a { }".chars().collect());
        assert_eq!(
            lexer.token().unwrap(),
            PToken {
                token: Token::Value(".a".to_string()),
                from: Cursor { row: 0, column: 0 },
                to: Cursor { row: 0, column: 1 }
            }
        );
        assert_eq!(
            lexer.token().unwrap(),
            PToken {
                token: Token::LBrace,
                from: Cursor { row: 0, column: 3 },
                to: Cursor { row: 0, column: 3 }
            }
        );
        assert_eq!(
            lexer.token().unwrap(),
            PToken {
                token: Token::RBrace,
                from: Cursor { row: 0, column: 5 },
                to: Cursor { row: 0, column: 5 }
            }
        );
        assert_eq!(lexer.token(), None);
    }

    #[test]
    fn multi() {
        let mut lexer = Lexer::new(".aa, .bb {".chars().collect());
        assert_eq!(
            lexer.token().unwrap().token,
            Token::Value(".aa".to_string())
        );
        assert_eq!(lexer.token().unwrap().token, Token::Comma);
        assert_eq!(
            lexer.token().unwrap().token,
            Token::Value(".bb".to_string())
        );
        assert_eq!(lexer.token().unwrap().token, Token::LBrace);
    }

    #[test]
    fn multi_line() {
        let mut lexer = Lexer::new(".a,\n.b {".chars().collect());
        assert_eq!(
            lexer.token().unwrap(),
            PToken {
                token: Token::Value(".a".to_string()),
                from: Cursor { row: 0, column: 0 },
                to: Cursor { row: 0, column: 1 }
            }
        );
        assert_eq!(
            lexer.token().unwrap(),
            PToken {
                token: Token::Comma,
                from: Cursor { row: 0, column: 2 },
                to: Cursor { row: 0, column: 2 }
            }
        );
        assert_eq!(
            lexer.token().unwrap(),
            PToken {
                token: Token::Value(".b".to_string()),
                from: Cursor { row: 1, column: 0 },
                to: Cursor { row: 1, column: 1 }
            }
        );
        assert_eq!(
            lexer.token().unwrap(),
            PToken {
                token: Token::LBrace,
                from: Cursor { row: 1, column: 3 },
                to: Cursor { row: 1, column: 3 }
            }
        );
    }

    #[test]
    fn nested() {
        let mut lexer = Lexer::new(".a .b {".chars().collect());
        assert_eq!(lexer.token().unwrap().token, Token::Value(".a".to_string()));
        assert_eq!(lexer.token().unwrap().token, Token::Value(".b".to_string()));
        assert_eq!(lexer.token().unwrap().token, Token::LBrace);
    }
}

#[cfg(test)]
mod property {
    use super::*;

    #[test]
    fn simple() {
        let mut lexer = Lexer::new("color: red;".chars().collect());
        assert_eq!(
            lexer.token().unwrap().token,
            Token::Value("color".to_string())
        );
        assert_eq!(lexer.token().unwrap().token, Token::Colon);
        assert_eq!(
            lexer.token().unwrap().token,
            Token::Value("red".to_string())
        );
        assert_eq!(lexer.token().unwrap().token, Token::Semicolon);
        assert_eq!(lexer.token(), None);
    }

    #[test]
    fn multi_value_online() {
        let mut lexer = Lexer::new("padding: 10px 1rem;".chars().collect());
        assert_eq!(
            lexer.token().unwrap().token,
            Token::Value("padding".to_string())
        );
        assert_eq!(lexer.token().unwrap().token, Token::Colon);
        assert_eq!(
            lexer.token().unwrap().token,
            Token::Value("10px".to_string())
        );
        assert_eq!(
            lexer.token().unwrap().token,
            Token::Value("1rem".to_string())
        );
        assert_eq!(lexer.token().unwrap().token, Token::Semicolon);
        assert_eq!(lexer.token(), None);
    }

    #[test]
    fn multi_value_multi_line() {
        let mut lexer = Lexer::new("padding: 10px\n1rem;".chars().collect());
        assert_eq!(
            lexer.token().unwrap().token,
            Token::Value("padding".to_string())
        );
        assert_eq!(lexer.token().unwrap().token, Token::Colon);
        assert_eq!(
            lexer.token().unwrap().token,
            Token::Value("10px".to_string())
        );
        assert_eq!(
            lexer.token().unwrap().token,
            Token::Value("1rem".to_string())
        );
        assert_eq!(lexer.token().unwrap().token, Token::Semicolon);
        assert_eq!(lexer.token(), None);
    }
}

#[cfg(test)]
mod line_comment {
    use super::*;

    #[test]
    fn test() {
        let mut lexer = Lexer::new(".a // abc \n.b".chars().collect());
        assert_eq!(lexer.token().unwrap().token, Token::Value(".a".to_string()));
        assert_eq!(
            lexer.token().unwrap().token,
            Token::Comment("// abc ".to_string())
        );
        assert_eq!(lexer.token().unwrap().token, Token::Value(".b".to_string()));
        assert_eq!(lexer.token(), None);
    }
}

#[cfg(test)]
mod block_comment {
    use super::*;

    #[test]
    fn online() {
        let mut lexer = Lexer::new(".a /* abc */ {".chars().collect());
        assert_eq!(lexer.token().unwrap().token, Token::Value(".a".to_string()));
        assert_eq!(
            lexer.token().unwrap().token,
            Token::Comment("/* abc */".to_string())
        );
        assert_eq!(lexer.token().unwrap().token, Token::LBrace);
        assert_eq!(lexer.token(), None);
    }

    #[test]
    fn multiline() {
        let mut lexer = Lexer::new(".a /*\n abc \n*/ {".chars().collect());
        assert_eq!(lexer.token().unwrap().token, Token::Value(".a".to_string()));
        assert_eq!(
            lexer.token().unwrap().token,
            Token::Comment("/*\n abc \n*/".to_string())
        );
        assert_eq!(lexer.token().unwrap().token, Token::LBrace);
        assert_eq!(lexer.token(), None);
    }
}
