use crate::lexer::Range;

#[derive(Debug, PartialEq, Clone)]
pub struct Scope {
    pub selectors: Vec<String>,
    pub children: Vec<Expr>,
    pub range: Range,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Property {
    pub key: String,
    pub value: String,
    pub range: Range,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Import {
    pub path: String,
    pub range: Range,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Include {
    pub name: String,
    pub range: Range,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Scope(Scope),
    Property(Property),
}
