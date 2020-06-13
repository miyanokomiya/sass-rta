use crate::lexer::Range;

#[derive(Debug, PartialEq, Clone)]
pub struct Scope {
    pub range: Range,
    pub selectors: Vec<String>,
    pub children: Vec<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Property {
    pub range: Range,
    pub key: String,
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Import {
    pub range: Range,
    pub path: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Include {
    pub range: Range,
    todo: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Mixin {
    pub range: Range,
    todo: String,
    pub children: Vec<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Media {
    pub range: Range,
    pub name: String,
    pub children: Vec<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Scope(Scope),
    Property(Property),
    Import(Import),
    Include(Include),
    Mixin(Mixin),
    Media(Media),
}
