use crate::lexer::Lexer;
use crate::lexer::PToken;

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

    fn parse(&mut self) -> &str {
        return "aa";
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        do_parser("1 + 2", "aa");
    }
    fn do_parser(input: &str, expect: &str) {
        let lexer = Lexer::new(input.chars().collect());
        let mut parser = Parser::new(lexer);
        assert_eq!(parser.parse(), expect);
    }
}
