use crate::errors::ShellError;
use crate::parser::parse2::{call_node::*, flag::*, operator::*, pipeline::*, span::*, tokens::*};
use crate::Text;
use derive_new::new;
use enum_utils::FromStr;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TokenNode {
    Token(Token),
    #[allow(unused)]
    Call(Spanned<CallNode>),
    Delimited(Spanned<DelimitedNode>),
    Pipeline(Spanned<Pipeline>),
    Operator(Spanned<Operator>),
    Flag(Spanned<Flag>),
    Identifier(Span),
    Whitespace(Span),
    #[allow(unused)]
    Error(Spanned<Box<ShellError>>),
    Path(Spanned<PathNode>),
}

impl TokenNode {
    pub fn span(&self) -> Span {
        match self {
            TokenNode::Token(t) => t.span,
            TokenNode::Call(s) => s.span,
            TokenNode::Delimited(s) => s.span,
            TokenNode::Pipeline(s) => s.span,
            TokenNode::Operator(s) => s.span,
            TokenNode::Flag(s) => s.span,
            TokenNode::Identifier(s) => *s,
            TokenNode::Whitespace(s) => *s,
            TokenNode::Error(s) => s.span,
            TokenNode::Path(s) => s.span,
        }
    }

    pub fn as_external_arg(&self, source: &Text) -> String {
        self.span().slice(source).to_string()
    }

    pub fn source(&self, source: &'a Text) -> &'a str {
        self.span().slice(source)
    }

    pub fn is_bare(&self) -> bool {
        match self {
            TokenNode::Token(Spanned {
                item: RawToken::Bare,
                ..
            }) => true,
            _ => false,
        }
    }

    crate fn as_flag(&self, value: &str, source: &Text) -> Option<Spanned<Flag>> {
        match self {
            TokenNode::Flag(
                flag @ Spanned {
                    item: Flag { .. }, ..
                },
            ) if value == flag.name().slice(source) => Some(*flag),
            _ => None,
        }
    }

    pub fn as_pipeline(&self) -> Result<Pipeline, ShellError> {
        match self {
            TokenNode::Pipeline(Spanned { item, .. }) => Ok(item.clone()),
            _ => Err(ShellError::string("unimplemented")),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, new)]
pub struct DelimitedNode {
    delimiter: Delimiter,
    children: Vec<TokenNode>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, FromStr)]
pub enum Delimiter {
    Paren,
    Brace,
    Square,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, new)]
pub struct PathNode {
    head: Box<TokenNode>,
    tail: Vec<TokenNode>,
}