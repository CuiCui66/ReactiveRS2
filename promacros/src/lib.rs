#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;

use std::vec::Vec;
use syntax::ptr::P;
use syntax::ast::{Expr};
use syntax::parse::token::*;
use syntax::tokenstream::*;

use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder; // A trait for expr_usize.
use syntax::ext::quote::rt::Span;
use rustc_plugin::Registry;

fn printtts(args : &[TokenTree]){
    for a in args {
        print!("{} ",a.clone().joint());
    }
    println!();
}

fn extract_ts(ts: TokenStream) -> Vec<TokenTree> {
    let mut res = vec![];
    let mut c = ts.trees();
    while let Some(tt) = c.next() {
        res.push(tt);
    }
    return res;
}

fn parse_expr(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> P<Expr> {
    let mut parser = cx.new_parser_from_tts(args);
    parser.parse_expr().unwrap_or_else(| mut d| {
        d.emit();
        return DummyResult::raw_expr(sp);
    }) // TODO handle error
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

fn split_on_binop_node(
    cx: &mut ExtCtxt,
    sp: Span,
    args: &[TokenTree],
    ind: usize,
) -> (P<Expr>, P<Expr>) {
    let (s1, s2tmp) = args.split_at(ind);
    let (_, s2) = s2tmp.split_at(1);
    let sp1 = sp.until(args[ind].span());
    let sp2 = args[ind].span().end_point().to(sp.end_point());

    (parse_node(cx, sp1, s1), parse_node(cx, sp2, s2))
}

fn parse_pro(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> P<Expr> {

    // print!("parse pro : ");
    // printtts(args);

    if args.len() == 0 {
        cx.expr_ident(sp,cx.ident_of("PNothing"));
    }
    if args.len() == 1 {
        match &args[0] {
            &TokenTree::Token(sp, ref tok) => {
                return parse_expr(cx, sp, args);
            }
            &TokenTree::Delimited(sp,
                                  Delimited {
                                      delim: d,
                                      tts: ref ts,
                                  }) => {
                if d == DelimToken::Paren || d == DelimToken::Brace {
                    return parse_pro(cx, sp, &extract_ts(ts.clone().into()));
                } else {
                    cx.span_err(sp, "Process delimited by brackets ?");
                    return DummyResult::raw_expr(sp);
                }
            }
        }
    }

    if args.len() == 3 {
        if let TokenTree::Token(_, Ident(id)) = args[0] {
            if id.name.as_str() == "choice" {
                let n1 = parse_pro(cx, args[1].span(), &args[1..2]);
                let n2 = parse_pro(cx, args[2].span(), &args[2..3]);
                return cx.expr_method_call(sp, n1, cx.ident_of("choice"), vec![n2]);
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

    parse_expr(cx, sp, args)
}

fn parse_node(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> P<Expr> {
    if args.len() == 0 {
        cx.expr_ident(sp,cx.ident_of("Nothing"));
    }
    if args.len() == 1 {
        match &args[0] {
            &TokenTree::Token(sp, ref tok) => {
                return parse_expr(cx, sp, args);
            }
            &TokenTree::Delimited(sp,
                                  Delimited {
                                      delim: d,
                                      tts: ref ts,
                                  }) => {
                if d == DelimToken::Paren || d == DelimToken::Brace {
                    return parse_node(cx, sp, &extract_ts(ts.clone().into()));
                } else {
                    cx.span_err(sp, "Process delimited by brackets ?");
                    return DummyResult::raw_expr(sp);
                }
            }
        }
    }

    // choice and like constructs
    if args.len() == 3 {
        if let TokenTree::Token(_, Ident(id)) = args[0] {
            if id.name.as_str() == "choice" {
                let n1 = parse_node(cx, args[1].span(), &args[1..2]);
                let n2 = parse_node(cx, args[2].span(), &args[2..3]);
                return cx.expr_method_call(sp, n1, cx.ident_of("alter"), vec![n2]);
            }
        }
    }

    for i in 0..args.len() {
        match args[i] {
            TokenTree::Token(spt, ref tok) => {
                match tok {
                    &Token::BinOp(Shr) => {
                        let (p1, p2) = split_on_binop_node(cx, sp, args, i);
                        return cx.expr_method_call(spt, p1, cx.ident_of("nseq"), vec![p2]);
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

    parse_expr(cx, sp, args)
}

fn expand_pro(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    MacEager::expr(parse_pro(cx, sp, args))
}

fn expand_node(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    MacEager::expr(parse_node(cx, sp, args))
}


#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("pro", expand_pro);
    reg.register_macro("node", expand_node);
}
