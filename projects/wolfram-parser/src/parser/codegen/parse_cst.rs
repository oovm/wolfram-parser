use super::*;

pub(super) fn parse_cst(input: &str, rule: WolframRule) -> OutputResult<WolframRule> {
    state(input, |state| match rule {
        WolframRule::Root => parse_root(state),
        WolframRule::Eos => parse_eos(state),
        WolframRule::Statements => parse_statements(state),
        WolframRule::Expression => parse_expression(state),
        WolframRule::Term => parse_term(state),
        WolframRule::Prefix => parse_prefix(state),
        WolframRule::Infix => parse_infix(state),
        WolframRule::InfixExpr => parse_infix_expr(state),
        WolframRule::GroupExpr => parse_group_expr(state),
        WolframRule::Suffix => parse_suffix(state),
        WolframRule::FunctionCall => parse_function_call(state),
        WolframRule::PartCall => parse_part_call(state),
        WolframRule::Atomic => parse_atomic(state),
        WolframRule::List => parse_list(state),
        WolframRule::Dict => parse_dict(state),
        WolframRule::Symbol => parse_symbol(state),
        WolframRule::IDENTIFIER => parse_identifier(state),
        WolframRule::String => parse_string(state),
        WolframRule::TEXT => parse_text(state),
        WolframRule::Integer => parse_integer(state),
        WolframRule::WhiteSpace => parse_white_space(state),
        WolframRule::Comment => parse_comment(state),
        WolframRule::HiddenText => unreachable!(),
    })
}
#[inline]
fn parse_root(state: Input) -> Output {
    state.rule(WolframRule::Root, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_statements(s).and_then(|s| s.tag_node("statements")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.end_of_input())
        })
    })
}
#[inline]
fn parse_eos(state: Input) -> Output {
    state.rule(WolframRule::Eos, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)([;])").unwrap())
        })
    })
}
#[inline]
fn parse_statements(state: Input) -> Output {
    state.rule(WolframRule::Statements, |s| Err(s).or_else(|s| parse_expression(s).and_then(|s| s.tag_node("expression"))))
}
#[inline]
fn parse_expression(state: Input) -> Output {
    state.rule(WolframRule::Expression, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| parse_term(s).and_then(|s| s.tag_node("term")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_eos(s).and_then(|s| s.tag_node("eos"))))
        })
    })
}
#[inline]
fn parse_term(state: Input) -> Output {
    state.rule(WolframRule::Term, |s| {
        Err(s)
            .or_else(|s| parse_prefix(s).and_then(|s| s.tag_node("prefix")))
            .or_else(|s| parse_infix(s).and_then(|s| s.tag_node("infix")))
            .or_else(|s| parse_infix_expr(s).and_then(|s| s.tag_node("infix_expr")))
            .or_else(|s| parse_group_expr(s).and_then(|s| s.tag_node("group_expr")))
            .or_else(|s| parse_atomic(s).and_then(|s| s.tag_node("atomic")))
            .or_else(|s| parse_suffix(s).and_then(|s| s.tag_node("suffix")))
            .or_else(|s| parse_function_call(s).and_then(|s| s.tag_node("function_call")))
            .or_else(|s| parse_part_call(s).and_then(|s| s.tag_node("part_call")))
    })
}
#[inline]
fn parse_prefix(state: Input) -> Output {
    state.rule(WolframRule::Prefix, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(!)").unwrap())
        })
    })
}
#[inline]
fn parse_infix(state: Input) -> Output {
    state.rule(WolframRule::Infix, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| {
                Regex::new(
                    "^(?x)(([^+*/]-)=?
    | [/]{2})",
                )
                .unwrap()
            })
        })
    })
}
#[inline]
fn parse_infix_expr(state: Input) -> Output {
    state.rule(WolframRule::InfixExpr, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "~", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_expression(s).and_then(|s| s.tag_node("expression")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "~", false))
        })
    })
}
#[inline]
fn parse_group_expr(state: Input) -> Output {
    state.rule(WolframRule::GroupExpr, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "(", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_expression(s).and_then(|s| s.tag_node("expression")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, ")", false))
        })
    })
}
#[inline]
fn parse_suffix(state: Input) -> Output {
    state.rule(WolframRule::Suffix, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(')").unwrap())
        })
    })
}
#[inline]
fn parse_function_call(state: Input) -> Output {
    state.rule(WolframRule::FunctionCall, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "[", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_expression(s).and_then(|s| s.tag_node("expression")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| {
                                s.sequence(|s| {
                                    Ok(s)
                                        .and_then(|s| builtin_text(s, ",", false))
                                        .and_then(|s| builtin_ignore(s))
                                        .and_then(|s| parse_expression(s).and_then(|s| s.tag_node("expression")))
                                })
                            })
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "]", false))
        })
    })
}
#[inline]
fn parse_part_call(state: Input) -> Output {
    state.rule(WolframRule::PartCall, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "[[", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_expression(s).and_then(|s| s.tag_node("expression")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| {
                                s.sequence(|s| {
                                    Ok(s)
                                        .and_then(|s| builtin_text(s, ",", false))
                                        .and_then(|s| builtin_ignore(s))
                                        .and_then(|s| parse_expression(s).and_then(|s| s.tag_node("expression")))
                                })
                            })
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "]]", false))
        })
    })
}
#[inline]
fn parse_atomic(state: Input) -> Output {
    state.rule(WolframRule::Atomic, |s| {
        Err(s)
            .or_else(|s| parse_list(s).and_then(|s| s.tag_node("list")))
            .or_else(|s| parse_dict(s).and_then(|s| s.tag_node("dict")))
            .or_else(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
            .or_else(|s| parse_integer(s).and_then(|s| s.tag_node("integer")))
    })
}
#[inline]
fn parse_list(state: Input) -> Output {
    state.rule(WolframRule::List, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "{", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_expression(s).and_then(|s| s.tag_node("expression")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| {
                                s.sequence(|s| {
                                    Ok(s)
                                        .and_then(|s| builtin_text(s, ",", false))
                                        .and_then(|s| builtin_ignore(s))
                                        .and_then(|s| parse_expression(s).and_then(|s| s.tag_node("expression")))
                                })
                            })
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "}", false))
        })
    })
}
#[inline]
fn parse_dict(state: Input) -> Output {
    state.rule(WolframRule::Dict, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "<|", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_expression(s).and_then(|s| s.tag_node("expression")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| {
                                s.sequence(|s| {
                                    Ok(s)
                                        .and_then(|s| builtin_text(s, ",", false))
                                        .and_then(|s| builtin_ignore(s))
                                        .and_then(|s| parse_expression(s).and_then(|s| s.tag_node("expression")))
                                })
                            })
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "|>", false))
        })
    })
}
#[inline]
fn parse_symbol(state: Input) -> Output {
    state.rule(WolframRule::Symbol, |s| {
        s.sequence(|s| {
            Ok(s).and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier"))).and_then(|s| {
                s.repeat(0..4294967295, |s| {
                    s.sequence(|s| {
                        Ok(s)
                            .and_then(|s| builtin_text(s, "`", false))
                            .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                    })
                })
            })
        })
    })
}
#[inline]
fn parse_identifier(state: Input) -> Output {
    state.rule(WolframRule::IDENTIFIER, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)([$\\p{XID_start}][$\\p{XID_continue}]*)").unwrap())
        })
    })
}
#[inline]
fn parse_string(state: Input) -> Output {
    state.rule(WolframRule::String, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "\"", false))
                .and_then(|s| s.repeat(0..4294967295, |s| parse_text(s).and_then(|s| s.tag_node("text"))))
                .and_then(|s| builtin_text(s, "\"", false))
        })
    })
}
#[inline]
fn parse_text(state: Input) -> Output {
    state.rule(WolframRule::TEXT, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)([^\"]*)").unwrap())
        })
    })
}
#[inline]
fn parse_integer(state: Input) -> Output {
    state.rule(WolframRule::Integer, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(0|[1-9][0-9]*)").unwrap())
        })
    })
}
#[inline]
fn parse_white_space(state: Input) -> Output {
    state.rule(WolframRule::WhiteSpace, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(\\p{White_Space}+)").unwrap())
        })
    })
}
#[inline]
fn parse_comment(state: Input) -> Output {
    state.rule(WolframRule::Comment, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "(*", false))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        Err(s)
                            .or_else(|s| parse_comment(s).and_then(|s| s.tag_node("comment")))
                            .or_else(|s| s.lookahead(false, |s| builtin_text(s, "*)", false)))
                            .or_else(|s| builtin_any(s))
                    })
                })
                .and_then(|s| builtin_text(s, "*)", false))
        })
    })
}

/// All rules ignored in ast mode, inline is not recommended
fn builtin_ignore(state: Input) -> Output {
    state.repeat(0..u32::MAX, |s| parse_white_space(s).or_else(|s| parse_comment(s)))
}

fn builtin_any(state: Input) -> Output {
    state.rule(WolframRule::HiddenText, |s| s.match_char_if(|_| true))
}

fn builtin_text<'i>(state: Input<'i>, text: &'static str, case: bool) -> Output<'i> {
    state.rule(WolframRule::HiddenText, |s| s.match_string(text, case))
}

fn builtin_regex<'i, 'r>(state: Input<'i>, regex: &'r Regex) -> Output<'i> {
    state.rule(WolframRule::HiddenText, |s| s.match_regex(regex))
}
