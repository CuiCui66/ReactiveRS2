#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;

use std::vec::Vec;
use syntax::ptr::P;
use syntax::ast::*;
use syntax::parse::token::*;
use syntax::tokenstream::*;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder; // A trait for expr_usize.
use syntax::ext::quote::rt::Span;
use rustc_plugin::Registry;


fn extract_ts(ts: TokenStream) -> Vec<TokenTree> {
    let mut res = vec![];
    let mut c = ts.trees();
    while let Some(tt) = c.next() {
        res.push(tt);
    }
    return res;
}

fn parse_expr(cx: &mut ExtCtxt,  args: &[TokenTree]) -> P<Expr> {
    let mut parser = cx.new_parser_from_tts(args);
    parser.parse_expr().unwrap() // TODO handle error
}

fn split_on_binop(
    cx: &mut ExtCtxt,
    sp: Span,
    args: &[TokenTree],
    ind: usize,
) -> (P<Expr>, P<Expr>) {
    let (s1, s2tmp) = args.split_at(ind);
    let (_, s2) = s2tmp.split_at(1);
    let sp1 = sp.until(args[ind].span());
    let sp2 = args[ind].span().end_point().to(sp.end_point());

    (parse_pro(cx, sp1, s1), parse_pro(cx, sp2, s2))
}

fn parse_pro(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> P<Expr> {
    if args.len() == 0 {
        cx.span_err(sp, "Empty expression");
        return DummyResult::raw_expr(sp);
    }
    if args.len() == 1 {
        match &args[0] {
            &TokenTree::Token(sp, ref tok) => {
                return parse_expr(cx, args);
            }
            &TokenTree::Delimited(sp,
                                  Delimited {
                                      delim: d,
                                      tts: ref ts,
                                  }) => {
                if d == DelimToken::Paren || d == DelimToken::Bracket {
                    return parse_pro(cx, sp, &extract_ts(ts.clone().into()));
                } else {
                    cx.span_err(sp, "Process delimited by brackets ?");
                    return DummyResult::raw_expr(sp);
                }
            }
        }
    }

    for i in 0..args.len() {
        match args[i] {
            TokenTree::Token(spt, ref tok) => {
                match tok {
                    &Token::Semi => {
                        let (p1, p2) = split_on_binop(cx, sp, args, i);
                        return cx.expr_method_call(spt, p1, cx.ident_of("seq"), vec![p2]);
                    }
                    _ => {}
                }
            }
            TokenTree::Delimited(sp,
                                 Delimited {
                                     delim: d,
                                     tts: ref ts,
                                 }) => {}
        }
    }

    parse_expr(cx,args)
}

fn expand_pro(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {

    /*static NUMERALS: &'static [(&'static str, usize)] = &[
        ("M", 1000), ("CM", 900), ("D", 500), ("CD", 400),
        ("C",  100), ("XC",  90), ("L",  50), ("XL",  40),
        ("X",   10), ("IX",   9), ("V",   5), ("IV",   4),
        ("I",    1)];

    if args.len() != 1 {
        cx.span_err(
            sp,
            &format!("argument should be a single identifier, but got {} arguments", args.len()));
        return DummyResult::any(sp);
    }

    let text = match args[0] {
        TokenTree::Token(_, token::Ident(s)) => s.to_string(),
        _ => {
            cx.span_err(sp, "argument should be a single identifier");
            return DummyResult::any(sp);
        }
    };

    let mut text = &*text;
    let mut total = 0;
    while !text.is_empty() {
        match NUMERALS.iter().find(|&&(rn, _)| text.starts_with(rn)) {
            Some(&(rn, val)) => {
                total += val;
                text = &text[rn.len()..];
            }
            None => {
                cx.span_err(sp, "invalid Roman numeral");
                return DummyResult::any(sp);
            }
        }
    }

    MacEager::expr(cx.expr_usize(sp, total))*/

    MacEager::expr(parse_pro(cx, sp, args))
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("ppro", expand_pro);
}
