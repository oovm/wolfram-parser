#![allow(dead_code, unused_imports, non_camel_case_types)]
#![allow(missing_docs, rustdoc::missing_crate_level_docs)]
#![allow(clippy::unnecessary_cast)]
#![doc = include_str!("readme.md")]

mod parse_ast;
mod parse_cst;

use core::str::FromStr;
use std::{borrow::Cow, ops::Range, sync::OnceLock};
use yggdrasil_rt::*;

type Input<'i> = Box<State<'i, WolframRule>>;
type Output<'i> = Result<Box<State<'i, WolframRule>>, Box<State<'i, WolframRule>>>;

#[doc = include_str!("railway.min.svg")]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WolframParser {}

impl YggdrasilParser for WolframParser {
    type Rule = WolframRule;
    fn parse_cst(input: &str, rule: Self::Rule) -> OutputResult<WolframRule> {
        self::parse_cst::parse_cst(input, rule)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WolframRule {
    Root,
    Eos,
    Statements,
    Expression,
    Term,
    Prefix,
    Infix,
    InfixExpr,
    GroupExpr,
    Suffix,
    FunctionCall,
    PartCall,
    Atomic,
    List,
    Dict,
    Symbol,
    IDENTIFIER,
    String,
    TEXT,
    Integer,
    WhiteSpace,
    Comment,
    /// Label for unnamed text literal
    HiddenText,
}

impl YggdrasilRule for WolframRule {
    fn is_ignore(&self) -> bool {
        matches!(self, Self::HiddenText | Self::WhiteSpace | Self::Comment)
    }

    fn get_style(&self) -> &'static str {
        match self {
            Self::Root => "",
            Self::Eos => "",
            Self::Statements => "",
            Self::Expression => "",
            Self::Term => "",
            Self::Prefix => "",
            Self::Infix => "",
            Self::InfixExpr => "",
            Self::GroupExpr => "",
            Self::Suffix => "",
            Self::FunctionCall => "",
            Self::PartCall => "",
            Self::Atomic => "",
            Self::List => "",
            Self::Dict => "",
            Self::Symbol => "",
            Self::IDENTIFIER => "",
            Self::String => "",
            Self::TEXT => "",
            Self::Integer => "",
            Self::WhiteSpace => "",
            Self::Comment => "",
            _ => "",
        }
    }
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RootNode {
    pub statements: Vec<StatementsNode>,
    pub span: Range<usize>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EosNode {
    pub span: Range<usize>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StatementsNode {
    Expression(ExpressionNode),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExpressionNode {
    pub eos: Option<EosNode>,
    pub term: Vec<TermNode>,
    pub span: Range<usize>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TermNode {
    Atomic(AtomicNode),
    FunctionCall(FunctionCallNode),
    GroupExpr(GroupExprNode),
    Infix(InfixNode),
    InfixExpr(InfixExprNode),
    PartCall(PartCallNode),
    Prefix(PrefixNode),
    Suffix(SuffixNode),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PrefixNode {
    pub text: String,
    pub span: Range<usize>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InfixNode {
    pub text: String,
    pub span: Range<usize>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InfixExprNode {
    pub expression: ExpressionNode,
    pub span: Range<usize>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GroupExprNode {
    pub expression: ExpressionNode,
    pub span: Range<usize>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SuffixNode {
    pub text: String,
    pub span: Range<usize>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionCallNode {
    pub expression: Vec<ExpressionNode>,
    pub span: Range<usize>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PartCallNode {
    pub expression: Vec<ExpressionNode>,
    pub span: Range<usize>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AtomicNode {
    Dict(DictNode),
    Identifier(IdentifierNode),
    Integer(IntegerNode),
    List(ListNode),
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListNode {
    pub expression: Vec<ExpressionNode>,
    pub span: Range<usize>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DictNode {
    pub expression: Vec<ExpressionNode>,
    pub span: Range<usize>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SymbolNode {
    pub identifier: Vec<IdentifierNode>,
    pub span: Range<usize>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdentifierNode {
    pub span: Range<usize>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StringNode {
    pub text: Vec<TextNode>,
    pub span: Range<usize>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextNode {
    pub span: Range<usize>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IntegerNode {
    pub span: Range<usize>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhiteSpaceNode {
    pub span: Range<usize>,
}
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CommentNode {
    pub comment: Vec<CommentNode>,
    pub span: Range<usize>,
}
