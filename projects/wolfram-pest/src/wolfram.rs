pub struct WolframParser;
#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    EOI,
    program,
    statement,
    emptyStatement,
    eos,
    comma_or_semi,
    apply,
    apply_kv,
    function_name,
    function_module,
    expression,
    expr,
    term,
    node,
    apply_call,
    derivative,
    space_call,
    data,
    dict,
    list,
    slice,
    key_value,
    index,
    Start,
    End,
    Step,
    Number,
    Decimal,
    Integer,
    BaseForm,
    ExDigits,
    Accuracy,
    Precision,
    Base,
    String,
    Symbol,
    SYMBOL,
    Pattern,
    Slot,
    Positive,
    WHITESPACE,
    COMMENT,
    Prefix,
    Suffix,
    Infix,
    Set,
    Derivative,
    Or,
    LazyOr,
    Star,
    Slash,
    Escape,
    Proportion,
    Comma,
    Dot,
    Separate,
    Semicolon,
    Colon,
    Question,
    Underline,
    Load,
    Save,
    LeftShift,
    RightShift,
    LessEqual,
    GraterEqual,
    Less,
    Grater,
    Pipeline,
    Rule,
    RuleDelayed,
    Equivalent,
    NotEquivalent,
    Equal,
    NotEqual,
    Plus,
    Minus,
    Multiply,
    CenterDot,
    Kronecker,
    TensorProduct,
    Divide,
    Quotient,
    Modulo,
    Remainder,
    Power,
    Surd,
    Increase,
    Decrease,
    To,
    Elvis,
    Map,
    Acute,
    Quote,
    LogicOr,
    LogicAnd,
    LogicNot,
    Ellipsis,
    LogicXor,
    MapAll,
    Concat,
    Destruct,
    DoubleBang,
    Bang,
    Not,
    Sharp,
    Curry,
    Apply,
    At,
}
#[allow(clippy::all)]
impl ::pest::Parser<Rule> for WolframParser {
    fn parse<'i>(rule: Rule, input: &'i str) -> ::std::result::Result<::pest::iterators::Pairs<'i, Rule>, ::pest::error::Error<Rule>> {
        mod rules {
            pub mod hidden {
                use super::super::Rule;
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn skip(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    if state.atomicity() == ::pest::Atomicity::NonAtomic { state.sequence(|state| state.repeat(|state| super::visible::WHITESPACE(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::visible::COMMENT(state).and_then(|state| state.repeat(|state| super::visible::WHITESPACE(state))))))) } else { Ok(state) }
                }
            }
            pub mod visible {
                use super::super::Rule;
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn program(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| self::SOI(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::statement(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::statement(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::EOI(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn statement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::emptyStatement(state).or_else(|state| self::expression(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn emptyStatement(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::emptyStatement, |state| self::eos(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn eos(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::eos, |state| self::Semicolon(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn comma_or_semi(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::Comma(state).or_else(|state| self::Semicolon(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn apply(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::apply, |state| state.sequence(|state| state.match_string("[").and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::apply_kv(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::apply_kv(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::apply_kv(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Comma(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("]")))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn apply_kv(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::apply_kv, |state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Colon(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state))).or_else(|state| self::expr(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn function_name(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::function_name, |state| self::SYMBOL(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn function_module(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::function_module, |state| state.sequence(|state| self::Dot(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Dot(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Dot(state))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn expression(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::expression, |state| state.sequence(|state| self::expr(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::eos(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn expr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::expr, |state| state.sequence(|state| self::term(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::Infix(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::term(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::Infix(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::term(state)))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn term(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::term, |state| state.sequence(|state| state.repeat(|state| self::Prefix(state)).and_then(|state| self::node(state)).and_then(|state| state.repeat(|state| self::Suffix(state).or_else(|state| state.sequence(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::WHITESPACE(state)))).and_then(|state| self::slice(state)))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn node(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::node, |state| state.sequence(|state| state.match_string("(").and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(")"))).or_else(|state| self::apply_call(state)).or_else(|state| self::derivative(state)).or_else(|state| self::space_call(state)).or_else(|state| self::data(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn apply_call(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::apply_call, |state| state.sequence(|state| self::Symbol(state).and_then(|state| state.sequence(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::WHITESPACE(state)))).and_then(|state| self::apply(state)))).and_then(|state| state.repeat(|state| state.sequence(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::WHITESPACE(state)))).and_then(|state| self::apply(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn derivative(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::derivative, |state| state.sequence(|state| self::Symbol(state).and_then(|state| self::Derivative(state)).and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::WHITESPACE(state))))).and_then(|state| self::apply(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn space_call(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::space_call, |state| state.sequence(|state| self::apply_call(state).or_else(|state| self::Number(state)).or_else(|state| self::Symbol(state)).and_then(|state| state.sequence(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::WHITESPACE(state)))).and_then(|state| self::apply_call(state).or_else(|state| self::Number(state)).or_else(|state| self::Symbol(state))))).and_then(|state| state.repeat(|state| state.sequence(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::WHITESPACE(state)))).and_then(|state| self::apply_call(state).or_else(|state| self::Number(state)).or_else(|state| self::Symbol(state)))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn data(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::data, |state| self::dict(state).or_else(|state| self::list(state)).or_else(|state| self::Slot(state)).or_else(|state| self::Number(state)).or_else(|state| self::String(state)).or_else(|state| self::Symbol(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn dict(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::dict, |state| state.sequence(|state| state.match_string("<|").and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("|>").or_else(|state| state.sequence(|state| self::key_value(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::key_value(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::key_value(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("|>"))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn list(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::list, |state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}").or_else(|state| state.sequence(|state| self::expr(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn slice(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::slice, |state| state.sequence(|state| state.match_string("[").and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("[")).and_then(|state| super::hidden::skip(state)).and_then(|state| self::index(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::index(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| self::Comma(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::index(state)))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("]")).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("]")))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn key_value(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::key_value, |state| state.sequence(|state| self::data(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::RuleDelayed(state).or_else(|state| self::Rule(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::expr(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn index(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::index, |state| state.sequence(|state| state.optional(|state| self::Start(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(";;")).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::End(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(";;")).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::Step(state)))).or_else(|state| state.sequence(|state| state.optional(|state| self::Start(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string(";;")).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::End(state))))).or_else(|state| self::expr(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Start(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Start, |state| self::expr(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn End(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::End, |state| self::expr(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Step(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Step, |state| self::expr(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Number(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Number, |state| state.sequence(|state| self::BaseForm(state).or_else(|state| self::Decimal(state)).or_else(|state| self::Integer(state)).and_then(|state| state.optional(|state| self::ExDigits(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Decimal(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Decimal, |state| state.sequence(|state| self::Dot(state).and_then(|state| self::Integer(state))).or_else(|state| state.sequence(|state| self::Integer(state).and_then(|state| self::Dot(state)).and_then(|state| state.optional(|state| self::Integer(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Integer(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Integer, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::ASCII_DIGIT(state).and_then(|state| state.repeat(|state| self::ASCII_DIGIT(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn BaseForm(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::BaseForm, |state| state.sequence(|state| self::Integer(state).and_then(|state| self::Base(state)).and_then(|state| self::ASCII_ALPHA(state).or_else(|state| self::ASCII_DIGIT(state))).and_then(|state| state.repeat(|state| self::ASCII_ALPHA(state).or_else(|state| self::ASCII_DIGIT(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn ExDigits(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::ExDigits, |state| state.sequence(|state| self::Accuracy(state).or_else(|state| self::Precision(state)).and_then(|state| self::Decimal(state).or_else(|state| self::Integer(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Accuracy(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Accuracy, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("``")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Precision(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Precision, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("`")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Base(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Base, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("^^")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn String(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::String, |state| state.sequence(|state| self::Quote(state).and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::Quote(state)).and_then(|state| self::ANY(state))))).and_then(|state| self::Quote(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Symbol(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Symbol, |state| state.sequence(|state| self::SYMBOL(state).and_then(|state| state.repeat(|state| state.sequence(|state| state.match_string("`").and_then(|state| self::SYMBOL(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SYMBOL(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SYMBOL, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::XID_START(state).and_then(|state| state.repeat(|state| self::XID_START(state).or_else(|state| self::ASCII_DIGIT(state))))).or_else(|state| state.sequence(|state| state.match_string("$").and_then(|state| state.repeat(|state| self::XID_START(state).or_else(|state| self::ASCII_DIGIT(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Pattern(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Pattern, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.repeat(|state| self::XID_START(state)).and_then(|state| self::Underline(state)).and_then(|state| state.repeat(|state| self::Underline(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Slot(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Slot, |state| state.sequence(|state| state.match_string("##").and_then(|state| state.optional(|state| self::Positive(state)))).or_else(|state| state.sequence(|state| state.match_string("#").and_then(|state| state.optional(|state| self::SYMBOL(state).or_else(|state| self::Integer(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Positive(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Positive, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| self::ASCII_NONZERO_DIGIT(state).and_then(|state| state.repeat(|state| self::ASCII_DIGIT(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn WHITESPACE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::Atomic, |state| self::NEWLINE(state).or_else(|state| self::WHITE_SPACE(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn COMMENT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.match_string("(*").and_then(|state| state.repeat(|state| self::COMMENT(state).or_else(|state| state.sequence(|state| state.lookahead(false, |state| state.match_string("*)")).and_then(|state| self::ANY(state)))))).and_then(|state| state.match_string("*)"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Prefix(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Prefix, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::Bang(state).or_else(|state| self::Not(state)).or_else(|state| self::Plus(state)).or_else(|state| self::Minus(state)).or_else(|state| self::Star(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Suffix(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Suffix, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::Derivative(state).or_else(|state| self::DoubleBang(state)).or_else(|state| self::Bang(state)).or_else(|state| self::Question(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Infix(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::Set(state).or_else(|state| self::Plus(state)).or_else(|state| self::Minus(state)).or_else(|state| self::Multiply(state)).or_else(|state| self::CenterDot(state)).or_else(|state| self::Kronecker(state)).or_else(|state| self::TensorProduct(state)).or_else(|state| self::Divide(state)).or_else(|state| self::Quotient(state)).or_else(|state| self::Modulo(state)).or_else(|state| self::Power(state)).or_else(|state| self::Grater(state)).or_else(|state| self::GraterEqual(state)).or_else(|state| self::Equal(state)).or_else(|state| self::Dot(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Set(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Set, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("=")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Derivative(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Derivative, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.sequence(|state| state.match_string("′").and_then(|state| state.repeat(|state| state.match_string("′"))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Or(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Or, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("|")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LazyOr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LazyOr, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("||")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Star(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Star, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("*")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Slash(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Slash, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("/")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Escape(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Escape, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("\\")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Proportion(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Proportion, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("::").or_else(|state| state.match_string("∷"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Comma(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Comma, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(",")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Dot(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Dot, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(".")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Separate(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Separate, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(";;")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Semicolon(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Semicolon, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(";")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Colon(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Colon, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(":")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Question(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Question, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("?")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Underline(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Underline, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("_")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Load(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Load, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("<<<").or_else(|state| state.match_string("⋘"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Save(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Save, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(">>>").or_else(|state| state.match_string("⋙"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LeftShift(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LeftShift, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("<<").or_else(|state| state.match_string("≪"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn RightShift(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::RightShift, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(">>").or_else(|state| state.match_string("≫"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LessEqual(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LessEqual, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("<=")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn GraterEqual(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::GraterEqual, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(">=")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Less(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Less, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("<")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Grater(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Grater, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(">")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Pipeline(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Pipeline, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("|>")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Rule(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Rule, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("->")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn RuleDelayed(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::RuleDelayed, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(":>")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Equivalent(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Equivalent, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("===")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NotEquivalent(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::NotEquivalent, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("=!=")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Equal(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Equal, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("==")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NotEqual(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::NotEqual, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("!=").or_else(|state| state.match_string("≠"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Plus(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Plus, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("+")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Minus(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Minus, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("-")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Multiply(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Multiply, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::Star(state).or_else(|state| state.match_string("×"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CenterDot(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::CenterDot, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("⋅")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Kronecker(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Kronecker, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("⊗")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn TensorProduct(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::TensorProduct, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("⊙")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Divide(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Divide, |state| state.atomic(::pest::Atomicity::Atomic, |state| self::Slash(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Quotient(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Quotient, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("//")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Modulo(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Modulo, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("%")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Remainder(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Remainder, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("÷")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Power(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Power, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("^")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Surd(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Surd, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("√")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Increase(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Increase, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("++")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Decrease(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Decrease, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("--")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn To(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::To, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("->")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Elvis(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Elvis, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(":?")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Map(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Map, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("/@")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Acute(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Acute, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("´")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Quote(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("\"")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LogicOr(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LogicOr, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("||").or_else(|state| state.match_string("∧"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LogicAnd(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LogicAnd, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("&&").or_else(|state| state.match_string("∨"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LogicNot(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LogicNot, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("¬")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Ellipsis(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Ellipsis, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("...").or_else(|state| state.match_string("…"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LogicXor(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LogicXor, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("⊕")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn MapAll(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::MapAll, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("//@")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Concat(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Concat, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("~~")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Destruct(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Destruct, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("~=")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn DoubleBang(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::DoubleBang, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("!!")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Bang(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Bang, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("!")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Not(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Not, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("¬")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Sharp(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Sharp, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("#")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Curry(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Curry, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("@@@")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Apply(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Apply, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("@@")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn At(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::At, |state| state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("@")))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ANY(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.skip(1)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn EOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::EOI, |state| state.end_of_input())
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn SOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.start_of_input()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_NONZERO_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('1'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_ALPHA(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('a'..'z').or_else(|state| state.match_range('A'..'Z'))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn NEWLINE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("\n").or_else(|state| state.match_string("\r\n")).or_else(|state| state.match_string("\r"))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn WHITE_SPACE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::WHITE_SPACE)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn XID_START(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::XID_START)
                }
            }
            pub use self::visible::*;
        }
        ::pest::state(input, |state| match rule {
            Rule::program => rules::program(state),
            Rule::statement => rules::statement(state),
            Rule::emptyStatement => rules::emptyStatement(state),
            Rule::eos => rules::eos(state),
            Rule::comma_or_semi => rules::comma_or_semi(state),
            Rule::apply => rules::apply(state),
            Rule::apply_kv => rules::apply_kv(state),
            Rule::function_name => rules::function_name(state),
            Rule::function_module => rules::function_module(state),
            Rule::expression => rules::expression(state),
            Rule::expr => rules::expr(state),
            Rule::term => rules::term(state),
            Rule::node => rules::node(state),
            Rule::apply_call => rules::apply_call(state),
            Rule::derivative => rules::derivative(state),
            Rule::space_call => rules::space_call(state),
            Rule::data => rules::data(state),
            Rule::dict => rules::dict(state),
            Rule::list => rules::list(state),
            Rule::slice => rules::slice(state),
            Rule::key_value => rules::key_value(state),
            Rule::index => rules::index(state),
            Rule::Start => rules::Start(state),
            Rule::End => rules::End(state),
            Rule::Step => rules::Step(state),
            Rule::Number => rules::Number(state),
            Rule::Decimal => rules::Decimal(state),
            Rule::Integer => rules::Integer(state),
            Rule::BaseForm => rules::BaseForm(state),
            Rule::ExDigits => rules::ExDigits(state),
            Rule::Accuracy => rules::Accuracy(state),
            Rule::Precision => rules::Precision(state),
            Rule::Base => rules::Base(state),
            Rule::String => rules::String(state),
            Rule::Symbol => rules::Symbol(state),
            Rule::SYMBOL => rules::SYMBOL(state),
            Rule::Pattern => rules::Pattern(state),
            Rule::Slot => rules::Slot(state),
            Rule::Positive => rules::Positive(state),
            Rule::WHITESPACE => rules::WHITESPACE(state),
            Rule::COMMENT => rules::COMMENT(state),
            Rule::Prefix => rules::Prefix(state),
            Rule::Suffix => rules::Suffix(state),
            Rule::Infix => rules::Infix(state),
            Rule::Set => rules::Set(state),
            Rule::Derivative => rules::Derivative(state),
            Rule::Or => rules::Or(state),
            Rule::LazyOr => rules::LazyOr(state),
            Rule::Star => rules::Star(state),
            Rule::Slash => rules::Slash(state),
            Rule::Escape => rules::Escape(state),
            Rule::Proportion => rules::Proportion(state),
            Rule::Comma => rules::Comma(state),
            Rule::Dot => rules::Dot(state),
            Rule::Separate => rules::Separate(state),
            Rule::Semicolon => rules::Semicolon(state),
            Rule::Colon => rules::Colon(state),
            Rule::Question => rules::Question(state),
            Rule::Underline => rules::Underline(state),
            Rule::Load => rules::Load(state),
            Rule::Save => rules::Save(state),
            Rule::LeftShift => rules::LeftShift(state),
            Rule::RightShift => rules::RightShift(state),
            Rule::LessEqual => rules::LessEqual(state),
            Rule::GraterEqual => rules::GraterEqual(state),
            Rule::Less => rules::Less(state),
            Rule::Grater => rules::Grater(state),
            Rule::Pipeline => rules::Pipeline(state),
            Rule::Rule => rules::Rule(state),
            Rule::RuleDelayed => rules::RuleDelayed(state),
            Rule::Equivalent => rules::Equivalent(state),
            Rule::NotEquivalent => rules::NotEquivalent(state),
            Rule::Equal => rules::Equal(state),
            Rule::NotEqual => rules::NotEqual(state),
            Rule::Plus => rules::Plus(state),
            Rule::Minus => rules::Minus(state),
            Rule::Multiply => rules::Multiply(state),
            Rule::CenterDot => rules::CenterDot(state),
            Rule::Kronecker => rules::Kronecker(state),
            Rule::TensorProduct => rules::TensorProduct(state),
            Rule::Divide => rules::Divide(state),
            Rule::Quotient => rules::Quotient(state),
            Rule::Modulo => rules::Modulo(state),
            Rule::Remainder => rules::Remainder(state),
            Rule::Power => rules::Power(state),
            Rule::Surd => rules::Surd(state),
            Rule::Increase => rules::Increase(state),
            Rule::Decrease => rules::Decrease(state),
            Rule::To => rules::To(state),
            Rule::Elvis => rules::Elvis(state),
            Rule::Map => rules::Map(state),
            Rule::Acute => rules::Acute(state),
            Rule::Quote => rules::Quote(state),
            Rule::LogicOr => rules::LogicOr(state),
            Rule::LogicAnd => rules::LogicAnd(state),
            Rule::LogicNot => rules::LogicNot(state),
            Rule::Ellipsis => rules::Ellipsis(state),
            Rule::LogicXor => rules::LogicXor(state),
            Rule::MapAll => rules::MapAll(state),
            Rule::Concat => rules::Concat(state),
            Rule::Destruct => rules::Destruct(state),
            Rule::DoubleBang => rules::DoubleBang(state),
            Rule::Bang => rules::Bang(state),
            Rule::Not => rules::Not(state),
            Rule::Sharp => rules::Sharp(state),
            Rule::Curry => rules::Curry(state),
            Rule::Apply => rules::Apply(state),
            Rule::At => rules::At(state),
            Rule::EOI => rules::EOI(state),
        })
    }
}
