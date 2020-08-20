use super::*;
#[automatically_derived]
impl YggdrasilNode for RootNode {
    type Rule = WolframRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            statements: pair.take_tagged_items::<StatementsNode>(Cow::Borrowed("statements")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for RootNode {
    type Err = YggdrasilError<WolframRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<WolframRule>> {
        Self::from_cst(WolframParser::parse_cst(input, WolframRule::Root)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for EosNode {
    type Rule = WolframRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for EosNode {
    type Err = YggdrasilError<WolframRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<WolframRule>> {
        Self::from_cst(WolframParser::parse_cst(input, WolframRule::Eos)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for StatementsNode {
    type Rule = WolframRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Expression(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<ExpressionNode>(Cow::Borrowed("expression")) {
            return Ok(Self::Expression(s));
        }
        Err(YggdrasilError::invalid_node(WolframRule::Statements, _span))
    }
}
#[automatically_derived]
impl FromStr for StatementsNode {
    type Err = YggdrasilError<WolframRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<WolframRule>> {
        Self::from_cst(WolframParser::parse_cst(input, WolframRule::Statements)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ExpressionNode {
    type Rule = WolframRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            eos: pair.take_tagged_option::<EosNode>(Cow::Borrowed("eos")),
            term: pair.take_tagged_items::<TermNode>(Cow::Borrowed("term")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for ExpressionNode {
    type Err = YggdrasilError<WolframRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<WolframRule>> {
        Self::from_cst(WolframParser::parse_cst(input, WolframRule::Expression)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TermNode {
    type Rule = WolframRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Atomic(s) => s.get_range(),
            Self::FunctionCall(s) => s.get_range(),
            Self::GroupExpr(s) => s.get_range(),
            Self::Infix(s) => s.get_range(),
            Self::InfixExpr(s) => s.get_range(),
            Self::PartCall(s) => s.get_range(),
            Self::Prefix(s) => s.get_range(),
            Self::Suffix(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<AtomicNode>(Cow::Borrowed("atomic")) {
            return Ok(Self::Atomic(s));
        }
        if let Ok(s) = pair.take_tagged_one::<FunctionCallNode>(Cow::Borrowed("function_call")) {
            return Ok(Self::FunctionCall(s));
        }
        if let Ok(s) = pair.take_tagged_one::<GroupExprNode>(Cow::Borrowed("group_expr")) {
            return Ok(Self::GroupExpr(s));
        }
        if let Ok(s) = pair.take_tagged_one::<InfixNode>(Cow::Borrowed("infix")) {
            return Ok(Self::Infix(s));
        }
        if let Ok(s) = pair.take_tagged_one::<InfixExprNode>(Cow::Borrowed("infix_expr")) {
            return Ok(Self::InfixExpr(s));
        }
        if let Ok(s) = pair.take_tagged_one::<PartCallNode>(Cow::Borrowed("part_call")) {
            return Ok(Self::PartCall(s));
        }
        if let Ok(s) = pair.take_tagged_one::<PrefixNode>(Cow::Borrowed("prefix")) {
            return Ok(Self::Prefix(s));
        }
        if let Ok(s) = pair.take_tagged_one::<SuffixNode>(Cow::Borrowed("suffix")) {
            return Ok(Self::Suffix(s));
        }
        Err(YggdrasilError::invalid_node(WolframRule::Term, _span))
    }
}
#[automatically_derived]
impl FromStr for TermNode {
    type Err = YggdrasilError<WolframRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<WolframRule>> {
        Self::from_cst(WolframParser::parse_cst(input, WolframRule::Term)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for PrefixNode {
    type Rule = WolframRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { text: pair.get_string(), span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for PrefixNode {
    type Err = YggdrasilError<WolframRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<WolframRule>> {
        Self::from_cst(WolframParser::parse_cst(input, WolframRule::Prefix)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for InfixNode {
    type Rule = WolframRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { text: pair.get_string(), span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for InfixNode {
    type Err = YggdrasilError<WolframRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<WolframRule>> {
        Self::from_cst(WolframParser::parse_cst(input, WolframRule::Infix)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for InfixExprNode {
    type Rule = WolframRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            expression: pair.take_tagged_one::<ExpressionNode>(Cow::Borrowed("expression"))?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for InfixExprNode {
    type Err = YggdrasilError<WolframRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<WolframRule>> {
        Self::from_cst(WolframParser::parse_cst(input, WolframRule::InfixExpr)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for GroupExprNode {
    type Rule = WolframRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            expression: pair.take_tagged_one::<ExpressionNode>(Cow::Borrowed("expression"))?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for GroupExprNode {
    type Err = YggdrasilError<WolframRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<WolframRule>> {
        Self::from_cst(WolframParser::parse_cst(input, WolframRule::GroupExpr)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for SuffixNode {
    type Rule = WolframRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { text: pair.get_string(), span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for SuffixNode {
    type Err = YggdrasilError<WolframRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<WolframRule>> {
        Self::from_cst(WolframParser::parse_cst(input, WolframRule::Suffix)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for FunctionCallNode {
    type Rule = WolframRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            expression: pair.take_tagged_items::<ExpressionNode>(Cow::Borrowed("expression")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for FunctionCallNode {
    type Err = YggdrasilError<WolframRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<WolframRule>> {
        Self::from_cst(WolframParser::parse_cst(input, WolframRule::FunctionCall)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for PartCallNode {
    type Rule = WolframRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            expression: pair.take_tagged_items::<ExpressionNode>(Cow::Borrowed("expression")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for PartCallNode {
    type Err = YggdrasilError<WolframRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<WolframRule>> {
        Self::from_cst(WolframParser::parse_cst(input, WolframRule::PartCall)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for AtomicNode {
    type Rule = WolframRule;

    fn get_range(&self) -> Option<Range<usize>> {
        match self {
            Self::Dict(s) => s.get_range(),
            Self::Integer(s) => s.get_range(),
            Self::List(s) => s.get_range(),
            Self::Symbol(s) => s.get_range(),
        }
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        if let Ok(s) = pair.take_tagged_one::<DictNode>(Cow::Borrowed("dict")) {
            return Ok(Self::Dict(s));
        }
        if let Ok(s) = pair.take_tagged_one::<IntegerNode>(Cow::Borrowed("integer")) {
            return Ok(Self::Integer(s));
        }
        if let Ok(s) = pair.take_tagged_one::<ListNode>(Cow::Borrowed("list")) {
            return Ok(Self::List(s));
        }
        if let Ok(s) = pair.take_tagged_one::<SymbolNode>(Cow::Borrowed("symbol")) {
            return Ok(Self::Symbol(s));
        }
        Err(YggdrasilError::invalid_node(WolframRule::Atomic, _span))
    }
}
#[automatically_derived]
impl FromStr for AtomicNode {
    type Err = YggdrasilError<WolframRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<WolframRule>> {
        Self::from_cst(WolframParser::parse_cst(input, WolframRule::Atomic)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for ListNode {
    type Rule = WolframRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            expression: pair.take_tagged_items::<ExpressionNode>(Cow::Borrowed("expression")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for ListNode {
    type Err = YggdrasilError<WolframRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<WolframRule>> {
        Self::from_cst(WolframParser::parse_cst(input, WolframRule::List)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for DictNode {
    type Rule = WolframRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            expression: pair.take_tagged_items::<ExpressionNode>(Cow::Borrowed("expression")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for DictNode {
    type Err = YggdrasilError<WolframRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<WolframRule>> {
        Self::from_cst(WolframParser::parse_cst(input, WolframRule::Dict)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for SymbolNode {
    type Rule = WolframRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            identifier: pair.take_tagged_items::<IdentifierNode>(Cow::Borrowed("identifier")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for SymbolNode {
    type Err = YggdrasilError<WolframRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<WolframRule>> {
        Self::from_cst(WolframParser::parse_cst(input, WolframRule::Symbol)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for IdentifierNode {
    type Rule = WolframRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { text: pair.get_string(), span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for IdentifierNode {
    type Err = YggdrasilError<WolframRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<WolframRule>> {
        Self::from_cst(WolframParser::parse_cst(input, WolframRule::IDENTIFIER)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for StringNode {
    type Rule = WolframRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            text: pair.take_tagged_items::<TextNode>(Cow::Borrowed("text")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for StringNode {
    type Err = YggdrasilError<WolframRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<WolframRule>> {
        Self::from_cst(WolframParser::parse_cst(input, WolframRule::String)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for TextNode {
    type Rule = WolframRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for TextNode {
    type Err = YggdrasilError<WolframRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<WolframRule>> {
        Self::from_cst(WolframParser::parse_cst(input, WolframRule::TEXT)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for IntegerNode {
    type Rule = WolframRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { text: pair.get_string(), span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for IntegerNode {
    type Err = YggdrasilError<WolframRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<WolframRule>> {
        Self::from_cst(WolframParser::parse_cst(input, WolframRule::Integer)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for WhiteSpaceNode {
    type Rule = WolframRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as usize, end: _span.end() as usize } })
    }
}
#[automatically_derived]
impl FromStr for WhiteSpaceNode {
    type Err = YggdrasilError<WolframRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<WolframRule>> {
        Self::from_cst(WolframParser::parse_cst(input, WolframRule::WhiteSpace)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for CommentNode {
    type Rule = WolframRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            comment: pair.take_tagged_items::<CommentNode>(Cow::Borrowed("comment")).collect::<Result<Vec<_>, _>>()?,
            span: Range { start: _span.start() as usize, end: _span.end() as usize },
        })
    }
}
#[automatically_derived]
impl FromStr for CommentNode {
    type Err = YggdrasilError<WolframRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<WolframRule>> {
        Self::from_cst(WolframParser::parse_cst(input, WolframRule::Comment)?)
    }
}
