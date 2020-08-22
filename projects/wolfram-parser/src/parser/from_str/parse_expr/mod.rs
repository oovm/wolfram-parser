use super::*;
use crate::{
    ast::WolframOperator,
    parser::codegen::{InfixNode, PrefixNode, SuffixNode, TermNode},
};
use pratt::{Affix, Associativity, PrattParser, Precedence};
use std::vec::IntoIter;

impl ExpressionNode {
    pub fn build(self, file: FileID) -> Result<WolframExpression> {
        match ExprResolver.parse(self.term.into_iter()) {
            Ok(o) => Ok(o.with_file(file)),
            Err(e) => {
                panic!("{e:?}")
            }
        }
    }
}

struct ExprResolver;

impl PrattParser<IntoIter<TermNode>> for ExprResolver {
    type Error = WolframError;
    type Input = TermNode;
    type Output = WolframExpression;

    fn query(&mut self, input: &Self::Input) -> std::result::Result<Affix, Self::Error> {
        let affix = match input {
            // prefix
            TermNode::Prefix(v) => v.as_operator().as_affix(),
            // infix
            TermNode::Infix(v) => v.as_operator().as_affix(),
            TermNode::InfixExpr(_) => Affix::Infix(Precedence(1000), Associativity::Left),
            // atom
            TermNode::GroupExpr(_) => Affix::Nilfix,
            TermNode::Atomic(_) => Affix::Nilfix,
            // suffix
            TermNode::Suffix(v) => v.as_operator().as_affix(),
            // TermNode::FunctionCall(_) => Affix::Postfix(Precedence(u32::MAX)),
            // TermNode::PartCall(_) => Affix::Postfix(Precedence(u32::MAX)),
        };
        Ok(affix)
    }

    fn primary(&mut self, input: Self::Input) -> std::result::Result<Self::Output, Self::Error> {
        let e = match input {
            TermNode::Atomic(v) => v.build(FileID::default())?,
            TermNode::GroupExpr(v) => v.expression.build(FileID::default())?,
            _ => unreachable!(),
        };
        Ok(e)
    }

    fn infix(
        &mut self,
        lhs: Self::Output,
        tree: Self::Input,
        rhs: Self::Output,
    ) -> std::result::Result<Self::Output, Self::Error> {
        let o = match tree {
            TermNode::Infix(v) => v.as_operator(),
            TermNode::InfixExpr(_) => {
                unimplemented!()
            }
            _ => unreachable!(),
        };
        Ok(WolframExpression::infix(lhs, o, rhs))
    }

    fn prefix(&mut self, tree: Self::Input, rhs: Self::Output) -> std::result::Result<Self::Output, Self::Error> {
        let o = match tree {
            TermNode::Prefix(v) => v.as_operator(),
            _ => unreachable!(),
        };
        Ok(WolframExpression::prefix(o, rhs))
    }

    fn postfix(&mut self, lhs: Self::Output, tree: Self::Input) -> std::result::Result<Self::Output, Self::Error> {
        let o = match tree {
            TermNode::Suffix(v) => v.as_operator(),
            // TermNode::FunctionCall(_) =>  unimplemented!(),
            // TermNode::PartCall(_) =>  unimplemented!(),
            _ => unreachable!(),
        };
        Ok(WolframExpression::suffix(lhs, o))
    }
}
impl PrefixNode {
    pub fn as_operator(&self) -> WolframOperator {
        match self.text.as_str() {
            "-" => WolframOperator::Minus,
            "!" => WolframOperator::Not,
            _ => unimplemented!("Unknown Infix: `{}`", self.text),
        }
    }
}

impl InfixNode {
    pub fn as_operator(&self) -> WolframOperator {
        match self.text.as_str() {
            "+" => WolframOperator::Plus,
            "-" => WolframOperator::Minus,
            "*" => WolframOperator::Times,
            "^" => WolframOperator::Power,
            "/" => WolframOperator::Divide,
            "/@" => WolframOperator::Map,
            _ => unimplemented!("Unknown Infix: `{}`", self.text),
        }
    }
}

impl SuffixNode {
    pub fn as_operator(&self) -> WolframOperator {
        match self.text.as_str() {
            "'" => unimplemented!("Derivative = 1000"),
            _ => unimplemented!("Unknown Infix: `{}`", self.text),
        }
    }
}
