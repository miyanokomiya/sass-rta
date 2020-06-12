extern crate regex;
use crate::lexer::Cursor;
use crate::parser::Expr;
use crate::parser::Scope;

use regex::Regex;

impl Scope {
    fn has_amp(&mut self) -> bool {
        false
    }
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"(& )|(&\t)|(&\.)|(&:)|(&#)|(&\+)|(\+&)|(&>)|(>&)|(&~)|(~&)|(&\[)").unwrap();
}

fn has_evil_amp(s: &str) -> bool {
    if !s.contains("&") {
        return false;
    }

    match RE.captures(s) {
        Some(_) => false,
        None => true,
    }
}

#[cfg(test)]
mod ambuster {
    use super::*;

    #[test]
    fn test_has_evil_amp_false() {
        assert_eq!(has_evil_amp(".a"), false);
        assert_eq!(has_evil_amp("& a a &"), false);
        assert_eq!(has_evil_amp("&#a #a&"), false);
        assert_eq!(has_evil_amp("&.a .a&"), false);
        assert_eq!(has_evil_amp("&:a a:&"), false);
        assert_eq!(has_evil_amp("&+a a+&"), false);
        assert_eq!(has_evil_amp("&>a a>&"), false);
        assert_eq!(has_evil_amp("&~a a~&"), false);
        assert_eq!(has_evil_amp("&[a"), false);
    }

    #[test]
    fn test_has_evil_amp_true() {
        assert_eq!(has_evil_amp("&a"), true);
        assert_eq!(has_evil_amp("&-a"), true);
        assert_eq!(has_evil_amp("&_a"), true);

        assert_eq!(has_evil_amp("a&"), true);
        assert_eq!(has_evil_amp("a-&"), true);
        assert_eq!(has_evil_amp("a_&"), true);
    }
}
