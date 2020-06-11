use crate::lexer::Lexer;
use crate::lexer::PToken;
use crate::lexer::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct Scope {
    selectors: Vec<String>,
    children: Vec<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Property {
    key: String,
    value: String,
}

#[derive(Debug, PartialEq, Clone)]
enum Expr {
    Scope(Scope),
    Property(Property),
}

struct Parser {
    lexer: Lexer,
    curr: Option<PToken>,
    peek: Option<PToken>,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Parser {
        let curr = lexer.token();
        let peek = lexer.token();
        Parser { lexer, curr, peek }
    }

    fn next(&mut self) {
        self.curr = self.peek.clone();
        self.peek = self.lexer.token();
    }

    fn parse(&mut self) -> Vec<Expr> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Vec<Expr> {
        let mut vec = vec![];

        while self.curr.is_some() {
            if self.curr.clone().unwrap().token == Token::RBrace {
                break;
            } else if self.is_property() {
                match self.parse_property() {
                    Some(p) => vec.push(Expr::Property(p)),
                    _ => (),
                }
            } else if self.is_scope() {
                match self.parse_scope() {
                    Some(s) => {
                        vec.push(Expr::Scope(s));
                    }
                    _ => (),
                }
            }
            self.next();
        }

        vec
    }

    fn parse_scope(&mut self) -> Option<Scope> {
        let mut curr = self.curr.clone();
        let mut selectors = vec![];
        let mut value = "".to_string();

        println!("curr: {:?}", curr);

        // parse selectors
        while curr.is_some() {
            match curr?.token {
                Token::Value(val) => value = value + &val + " ",
                Token::Comma => {
                    selectors.push(value.trim().to_string());
                    value = "".to_string();
                }
                Token::LBrace => {
                    selectors.push(value.trim().to_string());
                    break;
                }
                Token::Colon => value = value.trim().to_string() + ":",
                _ => break,
            };
            self.next();
            curr = self.curr.clone();
        }

        self.next();
        let children = self.parse_expression();

        Some(Scope {
            selectors,
            children,
        })
    }

    fn parse_property(&mut self) -> Option<Property> {
        match self.curr.clone()?.token {
            Token::Value(key) => {
                self.next();
                self.next(); // skip ':'
                let value = self.parse_property_value()?;
                let prop = Property { key, value };
                Some(prop)
            }
            _ => None,
        }
    }

    fn parse_property_value(&mut self) -> Option<String> {
        let mut curr = self.curr.clone();
        let mut value = "".to_string();

        while curr.is_some() {
            match curr?.token {
                Token::Value(val) => value = value + &val + " ",
                Token::Semicolon => break,
                Token::RBrace => break,
                _ => break,
            };
            self.next();
            curr = self.curr.clone();
        }

        Some(value.trim().to_string())
    }

    fn is_property(&mut self) -> bool {
        let mut lexer = self.lexer.clone();

        let mut pt = lexer.token();
        while pt.is_some() {
            match pt.unwrap().token {
                Token::Semicolon => return true,
                Token::LBrace => return false,
                _ => (),
            };
            pt = lexer.token();
        }

        false
    }

    fn is_scope(&mut self) -> bool {
        let mut lexer = self.lexer.clone();

        let mut pt = self.peek.clone();
        while pt.is_some() {
            match pt.unwrap().token {
                Token::Semicolon => return false,
                Token::LBrace => return true,
                _ => (),
            };
            pt = lexer.token();
        }

        false
    }

    fn next_not_value_token(&mut self) -> Option<Token> {
        let mut lexer = self.lexer.clone();

        let mut pt = lexer.token();
        while pt.is_some() {
            match pt?.token {
                Token::Value(_) => (),
                t => return Some(t),
            };
            pt = lexer.token();
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_parser(input: &str, expect: Vec<Expr>) {
        let lexer = Lexer::new(input.chars().collect());
        let mut parser = Parser::new(lexer);
        assert_eq!(parser.parse(), expect);
    }

    #[test]
    fn property() {
        do_parser(
            "color: red;\npadding: 1px 1rem; margin: 0 1px 2px;",
            vec![
                Expr::Property(Property {
                    key: "color".to_string(),
                    value: "red".to_string(),
                }),
                Expr::Property(Property {
                    key: "padding".to_string(),
                    value: "1px 1rem".to_string(),
                }),
                Expr::Property(Property {
                    key: "margin".to_string(),
                    value: "0 1px 2px".to_string(),
                }),
            ],
        );
    }

    #[test]
    fn variable() {
        do_parser(
            "$primary: #123456;",
            vec![Expr::Property(Property {
                key: "$primary".to_string(),
                value: "#123456".to_string(),
            })],
        );
    }

    #[cfg(test)]
    mod scope {
        use super::*;

        #[test]
        fn selectors_1() {
            do_parser(
                ".a {}\n.c {}",
                vec![
                    Expr::Scope(Scope {
                        selectors: vec![".a".to_string()],
                        children: vec![],
                    }),
                    Expr::Scope(Scope {
                        selectors: vec![".c".to_string()],
                        children: vec![],
                    }),
                ],
            );
        }

        #[test]
        fn selectors_2() {
            do_parser(
                ".a .b {}\n.c, .d {}",
                vec![
                    Expr::Scope(Scope {
                        selectors: vec![".a .b".to_string()],
                        children: vec![],
                    }),
                    Expr::Scope(Scope {
                        selectors: vec![".c".to_string(), ".d".to_string()],
                        children: vec![],
                    }),
                ],
            );
        }

        #[test]
        fn pseudo() {
            do_parser(
                ".a:b {} .cc::ff {}",
                vec![
                    Expr::Scope(Scope {
                        selectors: vec![".a:b".to_string()],
                        children: vec![],
                    }),
                    Expr::Scope(Scope {
                        selectors: vec![".cc::ff".to_string()],
                        children: vec![],
                    }),
                ],
            );
        }

        #[test]
        fn nested_selectors() {
            do_parser(
                ".a .b { .c, .d {} #e {} }",
                vec![Expr::Scope(Scope {
                    selectors: vec![".a .b".to_string()],
                    children: vec![
                        Expr::Scope(Scope {
                            selectors: vec![".c".to_string(), ".d".to_string()],
                            children: vec![],
                        }),
                        Expr::Scope(Scope {
                            selectors: vec!["#e".to_string()],
                            children: vec![],
                        }),
                    ],
                })],
            );
        }
    }
}
