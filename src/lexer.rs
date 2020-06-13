#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Value(String),   // selector property value
    Comment(String), // // comment /* comment */
    Comma,           // ,
    LBrace,          // {
    RBrace,          // }
    Colon,           // :
    Semicolon,       // ;
}

#[derive(Debug, PartialEq, Clone)]
pub struct PToken {
    pub token: Token,
    pub range: Range,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Cursor {
    pub row: usize,
    pub column: usize,
}
impl Cursor {
    pub fn new(row: usize, column: usize) -> Cursor {
        Cursor { row, column }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Range {
    pub from: Cursor,
    pub to: Cursor,
}
impl Range {
    pub fn new(from: Cursor, to: Cursor) -> Range {
        Range { from, to }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    row: usize,
    column: usize,
    escaping: bool,
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Lexer {
        Lexer {
            input,
            position: 0,
            row: 0,
            column: 0,
            escaping: false,
        }
    }

    pub fn token(&mut self) -> Option<PToken> {
        {
            self.skip_whitespace();
        }

        self.escaping = false;
        let from = self.curr_cursor();

        let token = if self.curr()? == &'\\' {
            self.escaping = true;
            self.token_value()?
        } else if self.curr()? == &'/' && self.peek()? == &'/' {
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
                &'\'' => self.token_single_quote_value()?,
                &'"' => self.token_double_quote_value()?,
                _ => self.token_value()?,
            }
        };

        let to = self.curr_cursor();
        self.next();
        return Some(PToken {
            token,
            range: Range::new(from, to),
        });
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

    fn token_enclosed_value(&mut self, closed: &char) -> Option<Token> {
        let mut value = self.curr()?.to_string();
        while self.peek().is_some() {
            self.next();
            value = value + &self.curr()?.to_string();

            if !self.escaping && self.curr() == Some(closed) {
                break;
            }

            if self.curr()? == &'\\' {
                self.escaping = !self.escaping;
            } else {
                self.escaping = false
            }
        }
        return Some(Token::Value(value));
    }

    fn token_single_quote_value(&mut self) -> Option<Token> {
        self.token_enclosed_value(&'\'')
    }

    fn token_double_quote_value(&mut self) -> Option<Token> {
        self.token_enclosed_value(&'"')
    }

    fn token_value(&mut self) -> Option<Token> {
        let mut value = self.curr()?.to_string();
        while self.peek().is_some() && (self.escaping || Self::is_value(self.peek().unwrap())) {
            self.next();

            if self.curr() == Some(&'\'') {
                value = match self.token_single_quote_value()? {
                    Token::Value(v) => value + &v,
                    _ => value,
                };
            } else if self.curr() == Some(&'"') {
                value = match self.token_double_quote_value()? {
                    Token::Value(v) => value + &v,
                    _ => value,
                };
            } else {
                value = value + &self.curr()?.to_string();
            }

            if self.curr()? == &'\\' {
                self.escaping = !self.escaping;
            } else {
                self.escaping = false
            }
        }
        return Some(Token::Value(value));
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
                range: Range::new(Cursor::new(0, 0), Cursor::new(0, 1)),
            }
        );
        assert_eq!(
            lexer.token().unwrap(),
            PToken {
                token: Token::LBrace,
                range: Range::new(Cursor::new(0, 3), Cursor::new(0, 3)),
            }
        );
        assert_eq!(
            lexer.token().unwrap(),
            PToken {
                token: Token::RBrace,
                range: Range::new(Cursor::new(0, 5), Cursor::new(0, 5)),
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
                range: Range::new(Cursor::new(0, 0), Cursor::new(0, 1)),
            }
        );
        assert_eq!(
            lexer.token().unwrap(),
            PToken {
                token: Token::Comma,
                range: Range::new(Cursor::new(0, 2), Cursor::new(0, 2)),
            }
        );
        assert_eq!(
            lexer.token().unwrap(),
            PToken {
                token: Token::Value(".b".to_string()),
                range: Range::new(Cursor::new(1, 0), Cursor::new(1, 1)),
            }
        );
        assert_eq!(
            lexer.token().unwrap(),
            PToken {
                token: Token::LBrace,
                range: Range::new(Cursor::new(1, 3), Cursor::new(1, 3)),
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

    #[test]
    fn escaped() {
        let mut lexer = Lexer::new(".a\\:b {".chars().collect());
        assert_eq!(
            lexer.token().unwrap().token,
            Token::Value(".a\\:b".to_string())
        );
        assert_eq!(lexer.token().unwrap().token, Token::LBrace);
    }

    #[test]
    fn prefix() {
        let mut lexer = Lexer::new(".a:hover {".chars().collect());
        assert_eq!(lexer.token().unwrap().token, Token::Value(".a".to_string()));
        assert_eq!(lexer.token().unwrap().token, Token::Colon);
        assert_eq!(
            lexer.token().unwrap().token,
            Token::Value("hover".to_string())
        );
        assert_eq!(lexer.token().unwrap().token, Token::LBrace);
    }

    #[test]
    fn pseudo() {
        let mut lexer = Lexer::new(".a::before {".chars().collect());
        assert_eq!(lexer.token().unwrap().token, Token::Value(".a".to_string()));
        assert_eq!(lexer.token().unwrap().token, Token::Colon);
        assert_eq!(lexer.token().unwrap().token, Token::Colon);
        assert_eq!(
            lexer.token().unwrap().token,
            Token::Value("before".to_string())
        );
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

    #[test]
    fn single_quote_value() {
        let mut lexer = Lexer::new("url('http://example.com')".chars().collect());
        assert_eq!(
            lexer.token().unwrap().token,
            Token::Value("url('http://example.com')".to_string())
        );
        assert_eq!(lexer.token(), None);
    }

    #[test]
    fn double_quote_value() {
        let mut lexer = Lexer::new("url(\"http://example.com\")".chars().collect());
        assert_eq!(
            lexer.token().unwrap().token,
            Token::Value("url(\"http://example.com\")".to_string())
        );
        assert_eq!(lexer.token(), None);
    }

    #[test]
    fn single_quote_escaped_value() {
        let mut lexer = Lexer::new("url('http://ex\\'ample.com')".chars().collect());
        assert_eq!(
            lexer.token().unwrap().token,
            Token::Value("url('http://ex\\'ample.com')".to_string())
        );
        assert_eq!(lexer.token(), None);
    }

    #[test]
    fn double_quote_escaped_value() {
        let mut lexer = Lexer::new("url(\"http://ex\\\"ample.com\")".chars().collect());
        assert_eq!(
            lexer.token().unwrap().token,
            Token::Value("url(\"http://ex\\\"ample.com\")".to_string())
        );
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
