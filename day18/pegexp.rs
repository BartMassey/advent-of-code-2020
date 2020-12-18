#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use peg;
enum Token {
    Plus,
    Times,
    LParen,
    RParen,
    Num(i64),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Token {
    fn fmt(
        &self,
        f: &mut ::core::fmt::Formatter,
    ) -> ::core::fmt::Result {
        match (&*self,) {
            (&Token::Plus,) => {
                let mut debug_trait_builder = f.debug_tuple("Plus");
                debug_trait_builder.finish()
            }
            (&Token::Times,) => {
                let mut debug_trait_builder = f.debug_tuple("Times");
                debug_trait_builder.finish()
            }
            (&Token::LParen,) => {
                let mut debug_trait_builder = f.debug_tuple("LParen");
                debug_trait_builder.finish()
            }
            (&Token::RParen,) => {
                let mut debug_trait_builder = f.debug_tuple("RParen");
                debug_trait_builder.finish()
            }
            (&Token::Num(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Num");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Token {
    #[inline]
    fn clone(&self) -> Token {
        {
            let _: ::core::clone::AssertParamIsClone<i64>;
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for Token {}
impl ::core::marker::StructuralPartialEq for Token {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for Token {
    #[inline]
    fn eq(&self, other: &Token) -> bool {
        {
            let __self_vi =
                ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi =
                ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (
                        &Token::Num(ref __self_0),
                        &Token::Num(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    _ => true,
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &Token) -> bool {
        {
            let __self_vi =
                ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi =
                ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (
                        &Token::Num(ref __self_0),
                        &Token::Num(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    _ => false,
                }
            } else {
                true
            }
        }
    }
}
impl ::core::marker::StructuralEq for Token {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for Token {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<i64>;
        }
    }
}
use Token::*;
mod eval {
    #[allow(unused_imports)]
    use super::*;
    type Input = [Token];
    type PositionRepr = <Input as ::peg::Parse>::PositionRepr;
    struct ParseState<'input> {
        _phantom: ::std::marker::PhantomData<&'input ()>,
    }
    impl<'input> ParseState<'input> {
        fn new() -> ParseState<'input> {
            ParseState {
                _phantom: ::std::marker::PhantomData,
            }
        }
    }
    fn __parse_number<'input>(
        __input: &'input Input,
        __state: &mut ParseState<'input>,
        __err_state: &mut ::peg::error::ErrorState,
        __pos: usize,
    ) -> ::peg::RuleResult<i64> {
        #![allow(non_snake_case, unused)]
        {
            let __seq_res =
                match ::peg::ParseElem::parse_elem(__input, __pos) {
                    ::peg::RuleResult::Matched(__next, __ch) => {
                        match __ch {
                            Token::Num(_) => {
                                ::peg::RuleResult::Matched(__next, ())
                            }
                            _ => __err_state
                                .mark_failure(__pos, "Token :: Num(_)"),
                        }
                    }
                    ::peg::RuleResult::Failed => __err_state
                        .mark_failure(__pos, "Token :: Num(_)"),
                };
            match __seq_res {
                ::peg::RuleResult::Matched(__pos, n) => {
                    ::peg::RuleResult::Matched(__pos, { n.0 })
                }
                ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
            }
        }
    }
    pub fn arithmetic<'input>(
        __input: &'input Input,
    ) -> ::std::result::Result<
        i64,
        ::peg::error::ParseError<PositionRepr>,
    > {
        #![allow(non_snake_case, unused)]
        let mut __err_state =
            ::peg::error::ErrorState::new(::peg::Parse::start(__input));
        let mut __state = ParseState::new();
        match __parse_arithmetic(
            __input,
            &mut __state,
            &mut __err_state,
            ::peg::Parse::start(__input),
        ) {
            ::peg::RuleResult::Matched(__pos, __value) => {
                if __pos == __input.len() {
                    return Ok(__value);
                } else {
                    __err_state.mark_failure(__pos, "EOF");
                }
            }
            _ => (),
        }
        __state = ParseState::new();
        __err_state.reparse_for_error();
        match __parse_arithmetic(
            __input,
            &mut __state,
            &mut __err_state,
            ::peg::Parse::start(__input),
        ) {
            ::peg::RuleResult::Matched(__pos, __value) => {
                if __pos == __input.len() {
                    {
                        :: std :: rt :: begin_panic ("Parser is nondeterministic: succeeded when reparsing for error position")
                    };
                } else {
                    __err_state.mark_failure(__pos, "EOF");
                }
            }
            _ => (),
        }
        Err(__err_state.into_parse_error(__input))
    }
    fn __parse_arithmetic<'input>(
        __input: &'input Input,
        __state: &mut ParseState<'input>,
        __err_state: &mut ::peg::error::ErrorState,
        __pos: usize,
    ) -> ::peg::RuleResult<i64> {
        #![allow(non_snake_case, unused)]
        {
            fn __infix_parse<T, S>(
                state: &mut S,
                err_state: &mut ::peg::error::ErrorState,
                min_prec: i32,
                lpos: usize,
                prefix_atom: &Fn(
                    usize,
                    &mut S,
                    &mut ::peg::error::ErrorState,
                    &Fn(
                        usize,
                        i32,
                        &mut S,
                        &mut ::peg::error::ErrorState,
                    ) -> ::peg::RuleResult<T>,
                )
                    -> ::peg::RuleResult<T>,
                level_code: &Fn(
                    usize,
                    usize,
                    i32,
                    T,
                    &mut S,
                    &mut ::peg::error::ErrorState,
                    &Fn(
                        usize,
                        i32,
                        &mut S,
                        &mut ::peg::error::ErrorState,
                    ) -> ::peg::RuleResult<T>,
                )
                    -> (T, ::peg::RuleResult<()>),
            ) -> ::peg::RuleResult<T> {
                let initial = {
                    prefix_atom(
                        lpos,
                        state,
                        err_state,
                        &(|pos, min_prec, state, err_state| {
                            __infix_parse(
                                state,
                                err_state,
                                min_prec,
                                pos,
                                prefix_atom,
                                level_code,
                            )
                        }),
                    )
                };
                if let ::peg::RuleResult::Matched(
                    pos,
                    mut infix_result,
                ) = initial
                {
                    let mut repeat_pos = pos;
                    loop {
                        let (val, res) = level_code(
                            repeat_pos,
                            lpos,
                            min_prec,
                            infix_result,
                            state,
                            err_state,
                            &(|pos, min_prec, state, err_state| {
                                __infix_parse(
                                    state,
                                    err_state,
                                    min_prec,
                                    pos,
                                    prefix_atom,
                                    level_code,
                                )
                            }),
                        );
                        infix_result = val;
                        if let ::peg::RuleResult::Matched(pos, ()) = res
                        {
                            repeat_pos = pos;
                            continue;
                        }
                        break;
                    }
                    ::peg::RuleResult::Matched(repeat_pos, infix_result)
                } else {
                    ::peg::RuleResult::Failed
                }
            }
            __infix_parse(
                __state,
                __err_state,
                0,
                __pos,
                &(|__pos, __state, __err_state, __recurse| {
                    let __lpos = __pos;
                    if let ::peg::RuleResult::Matched(__pos, __v) = {
                        let __seq_res = __parse_number(
                            __input,
                            __state,
                            __err_state,
                            __pos,
                        );
                        match __seq_res {
                            ::peg::RuleResult::Matched(__pos, n) => {
                                ::peg::RuleResult::Matched(__pos, { n })
                            }
                            ::peg::RuleResult::Failed => {
                                ::peg::RuleResult::Failed
                            }
                        }
                    } {
                        return ::peg::RuleResult::Matched(__pos, __v);
                    }
                    if let ::peg::RuleResult::Matched(__pos, __v) = {
                        let __seq_res =
                            match ::peg::ParseElem::parse_elem(
                                __input, __pos,
                            ) {
                                ::peg::RuleResult::Matched(
                                    __next,
                                    __ch,
                                ) => match __ch {
                                    Token::LParen => {
                                        ::peg::RuleResult::Matched(
                                            __next,
                                            (),
                                        )
                                    }
                                    _ => __err_state.mark_failure(
                                        __pos,
                                        "Token :: LParen",
                                    ),
                                },
                                ::peg::RuleResult::Failed => {
                                    __err_state.mark_failure(
                                        __pos,
                                        "Token :: LParen",
                                    )
                                }
                            };
                        match __seq_res {
                            ::peg::RuleResult::Matched(__pos, _) => {
                                let __seq_res = __parse_arithmetic(
                                    __input,
                                    __state,
                                    __err_state,
                                    __pos,
                                );
                                match __seq_res {
                                    ::peg::RuleResult::Matched(
                                        __pos,
                                        e,
                                    ) => {
                                        let __seq_res =
                                            match ::peg::ParseElem::parse_elem(__input, __pos) {
                                                ::peg::RuleResult::Matched(__next, __ch) => {
                                                    match __ch {
                                                        Token::RParen => {
                                                            ::peg::RuleResult::Matched(__next, ())
                                                        }
                                                        _ => __err_state
                                                            .mark_failure(__pos, "Token :: RParen"),
                                                    }
                                                }
                                                ::peg::RuleResult::Failed => __err_state
                                                    .mark_failure(__pos, "Token :: RParen"),
                                            };
                                        match __seq_res {
                                            ::peg::RuleResult::Matched(__pos, _) => {
                                                ::peg::RuleResult::Matched(__pos, { e })
                                            }
                                            ::peg::RuleResult::Failed => ::peg::RuleResult::Failed,
                                        }
                                    }
                                    ::peg::RuleResult::Failed => {
                                        ::peg::RuleResult::Failed
                                    }
                                }
                            }
                            ::peg::RuleResult::Failed => {
                                ::peg::RuleResult::Failed
                            }
                        }
                    } {
                        return ::peg::RuleResult::Matched(__pos, __v);
                    }
                    ::peg::RuleResult::Failed
                }),
                &(|__pos,
                   __lpos,
                   __min_prec,
                   mut __infix_result,
                   __state,
                   __err_state,
                   __recurse| {
                    if 0i32 >= __min_prec {
                        if let ::peg::RuleResult::Matched(__pos, ()) = {
                            let __seq_res =
                                match ::peg::ParseElem::parse_elem(
                                    __input, __pos,
                                ) {
                                    ::peg::RuleResult::Matched(
                                        __next,
                                        __ch,
                                    ) => match __ch {
                                        Token::Plus => {
                                            ::peg::RuleResult::Matched(
                                                __next,
                                                (),
                                            )
                                        }
                                        _ => __err_state.mark_failure(
                                            __pos,
                                            "Token :: Plus",
                                        ),
                                    },
                                    ::peg::RuleResult::Failed => {
                                        __err_state.mark_failure(
                                            __pos,
                                            "Token :: Plus",
                                        )
                                    }
                                };
                            match __seq_res {
                                ::peg::RuleResult::Matched(
                                    __pos,
                                    _,
                                ) => {
                                    if let ::peg::RuleResult::Matched(
                                        __pos,
                                        y,
                                    ) = __recurse(
                                        __pos,
                                        1i32,
                                        __state,
                                        __err_state,
                                    ) {
                                        let x = __infix_result;
                                        __infix_result = { x + y };
                                        ::peg::RuleResult::Matched(
                                            __pos,
                                            (),
                                        )
                                    } else {
                                        ::peg::RuleResult::Failed
                                    }
                                }
                                ::peg::RuleResult::Failed => {
                                    ::peg::RuleResult::Failed
                                }
                            }
                        } {
                            return (
                                __infix_result,
                                ::peg::RuleResult::Matched(__pos, ()),
                            );
                        }
                    }
                    if 1i32 >= __min_prec {
                        if let ::peg::RuleResult::Matched(__pos, ()) = {
                            let __seq_res =
                                match ::peg::ParseElem::parse_elem(
                                    __input, __pos,
                                ) {
                                    ::peg::RuleResult::Matched(
                                        __next,
                                        __ch,
                                    ) => match __ch {
                                        Token::Times => {
                                            ::peg::RuleResult::Matched(
                                                __next,
                                                (),
                                            )
                                        }
                                        _ => __err_state.mark_failure(
                                            __pos,
                                            "Token :: Times",
                                        ),
                                    },
                                    ::peg::RuleResult::Failed => {
                                        __err_state.mark_failure(
                                            __pos,
                                            "Token :: Times",
                                        )
                                    }
                                };
                            match __seq_res {
                                ::peg::RuleResult::Matched(
                                    __pos,
                                    _,
                                ) => {
                                    if let ::peg::RuleResult::Matched(
                                        __pos,
                                        y,
                                    ) = __recurse(
                                        __pos,
                                        2i32,
                                        __state,
                                        __err_state,
                                    ) {
                                        let x = __infix_result;
                                        __infix_result = { x * y };
                                        ::peg::RuleResult::Matched(
                                            __pos,
                                            (),
                                        )
                                    } else {
                                        ::peg::RuleResult::Failed
                                    }
                                }
                                ::peg::RuleResult::Failed => {
                                    ::peg::RuleResult::Failed
                                }
                            }
                        } {
                            return (
                                __infix_result,
                                ::peg::RuleResult::Matched(__pos, ()),
                            );
                        }
                    }
                    (__infix_result, ::peg::RuleResult::Failed)
                }),
            )
        }
    }
}
fn main() {
    let test = [Num(3), Times, Num(2)];
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["", "\n"],
            &match (&eval::arithmetic(&test).unwrap(),) {
                (arg0,) => [::core::fmt::ArgumentV1::new(
                    arg0,
                    ::core::fmt::Display::fmt,
                )],
            },
        ));
    };
}
