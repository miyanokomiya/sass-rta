use crate::expression::Expr;
use crate::expression::Import;
use crate::expression::Include;
use crate::expression::Property;
use crate::expression::Scope;
use crate::lexer::Cursor;
use crate::lexer::Lexer;
use crate::lexer::PToken;
use crate::lexer::Range;
use crate::lexer::Token;

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

        let from = self.curr.clone()?.range.from;

        // parse selectors
        while curr.clone().is_some() {
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

        let to = self.curr.clone()?.range.from;

        Some(Scope {
            selectors,
            children,
            range: Range::new(from, to),
        })
    }

    fn parse_property(&mut self) -> Option<Property> {
        let from = self.curr.clone()?.range.from;
        match self.curr.clone()?.token {
            Token::Value(key) => {
                self.next();
                self.next(); // skip ':'
                let value = self.parse_property_value()?;
                let to = self.curr.clone()?.range.from;
                let prop = Property {
                    key,
                    value,
                    range: Range::new(from, to),
                };
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

        let mut pt = self.peek.clone();
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

        let mut pt = self.peek.clone();
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
mod parser {
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
                    range: Range::new(Cursor::new(0, 0), Cursor::new(0, 10)),
                }),
                Expr::Property(Property {
                    key: "padding".to_string(),
                    value: "1px 1rem".to_string(),
                    range: Range::new(Cursor::new(1, 0), Cursor::new(1, 17)),
                }),
                Expr::Property(Property {
                    key: "margin".to_string(),
                    value: "0 1px 2px".to_string(),
                    range: Range::new(Cursor::new(1, 19), Cursor::new(1, 36)),
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
                range: Range::new(Cursor::new(0, 0), Cursor::new(0, 17)),
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
                        range: Range::new(Cursor::new(0, 0), Cursor::new(0, 4)),
                    }),
                    Expr::Scope(Scope {
                        selectors: vec![".c".to_string()],
                        children: vec![],
                        range: Range::new(Cursor::new(1, 0), Cursor::new(1, 4)),
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
                        range: Range::new(Cursor::new(0, 0), Cursor::new(0, 7)),
                    }),
                    Expr::Scope(Scope {
                        selectors: vec![".c".to_string(), ".d".to_string()],
                        children: vec![],
                        range: Range::new(Cursor::new(1, 0), Cursor::new(1, 8)),
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
                        range: Range::new(Cursor::new(0, 0), Cursor::new(0, 6)),
                    }),
                    Expr::Scope(Scope {
                        selectors: vec![".cc::ff".to_string()],
                        children: vec![],
                        range: Range::new(Cursor::new(0, 8), Cursor::new(0, 17)),
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
                            range: Range::new(Cursor::new(0, 8), Cursor::new(0, 16)),
                        }),
                        Expr::Scope(Scope {
                            selectors: vec!["#e".to_string()],
                            children: vec![],
                            range: Range::new(Cursor::new(0, 18), Cursor::new(0, 22)),
                        }),
                    ],
                    range: Range::new(Cursor::new(0, 0), Cursor::new(0, 24)),
                })],
            );
        }

        #[test]
        fn with_property() {
            do_parser(
                ".a { color: red; .b { width: 100px; } }",
                vec![Expr::Scope(Scope {
                    selectors: vec![".a".to_string()],
                    children: vec![
                        Expr::Property(Property {
                            key: "color".to_string(),
                            value: "red".to_string(),
                            range: Range::new(Cursor::new(0, 5), Cursor::new(0, 15)),
                        }),
                        Expr::Scope(Scope {
                            selectors: vec![".b".to_string()],
                            children: vec![Expr::Property(Property {
                                key: "width".to_string(),
                                value: "100px".to_string(),
                                range: Range::new(Cursor::new(0, 22), Cursor::new(0, 34)),
                            })],
                            range: Range::new(Cursor::new(0, 17), Cursor::new(0, 36)),
                        }),
                    ],
                    range: Range::new(Cursor::new(0, 0), Cursor::new(0, 38)),
                })],
            );
        }
    }
}
