use super::*;
use crate::parser::codegen::TermNode;
use pratt::{Affix, PrattError, PrattParser, Precedence};
use std::vec::IntoIter;

impl ExpressionNode {
    pub fn build(self, _: FileID) -> Result<WolframExpression> {
        match ExprResolver.parse(self.term.into_iter()) {
            Ok(o) => Ok(o),
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
            TermNode::Prefix(_) => Affix::Prefix(Precedence(1000)),
            // infix
            TermNode::Infix(_) => {}
            TermNode::InfixExpr(_) => {}
            // atom
            TermNode::GroupExpr(_) => Affix::Nilfix,
            TermNode::Atomic(_) => Affix::Nilfix,
            // suffix
            TermNode::Suffix(_) => Affix::Postfix(Precedence(1000)),
            TermNode::FunctionCall(_) => Affix::Postfix(Precedence(u32::MAX)),
            TermNode::PartCall(_) => Affix::Postfix(Precedence(u32::MAX)),
        };
        Ok(affix)
    }

    fn primary(&mut self, input: Self::Input) -> std::result::Result<Self::Output, Self::Error> {
        todo!()
    }

    fn infix(
        &mut self,
        lhs: Self::Output,
        op: Self::Input,
        rhs: Self::Output,
    ) -> std::result::Result<Self::Output, Self::Error> {
        todo!()
    }

    fn prefix(&mut self, op: Self::Input, rhs: Self::Output) -> std::result::Result<Self::Output, Self::Error> {
        todo!()
    }

    fn postfix(&mut self, lhs: Self::Output, op: Self::Input) -> std::result::Result<Self::Output, Self::Error> {
        todo!()
    }
}
