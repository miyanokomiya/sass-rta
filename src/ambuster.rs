extern crate regex;
use crate::lexer::Cursor;
use crate::parser::Expr;
use crate::parser::Scope;

use regex::Regex;

impl Scope {
    fn has_evil_amp(&mut self) -> bool {
        self.selectors.iter().find(|s| has_evil_amp(s)).is_some()
    }
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"([a-zA-Z0-9_\-]&)|(&[a-zA-Z0-9_\-])").unwrap();
}

fn has_evil_amp(s: &str) -> bool {
    match RE.captures(s) {
        Some(_) => true,
        None => false,
    }
}

#[cfg(test)]
mod ambuster {
    use super::*;

    #[test]
    fn test_has_evil_amp_false() {
        assert_eq!(has_evil_amp(".a"), false);
        assert_eq!(has_evil_amp("& a a &"), false);
        assert_eq!(has_evil_amp("&#a"), false);
        assert_eq!(has_evil_amp("&.a"), false);
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
