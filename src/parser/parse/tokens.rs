use crate::parser::parse::unit::*;
use crate::prelude::*;
use crate::{Span, Tagged, Text};
use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RawToken {
    Number(Number),
    Size(Number, Unit),
    String(Span),
    Variable(Span),
    External(Span),
    Bare,
}

impl RawToken {
    pub fn type_name(&self) -> &'static str {
        match self {
            RawToken::Number(_) => "Number",
            RawToken::Size(..) => "Size",
            RawToken::String(_) => "String",
            RawToken::Variable(_) => "Variable",
            RawToken::External(_) => "External",
            RawToken::Bare => "String",
        }
    }
}

pub type Token = Tagged<RawToken>;

impl Token {
    pub fn debug<'a>(&self, source: &'a Text) -> DebugToken<'a> {
        DebugToken {
            node: *self,
            source,
        }
    }
}

pub struct DebugToken<'a> {
    node: Token,
    source: &'a Text,
}

impl fmt::Debug for DebugToken<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.node.span().slice(self.source))
    }
}
