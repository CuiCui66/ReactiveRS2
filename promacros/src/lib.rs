#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;

use std::vec::Vec;
use syntax::ptr::P;
use syntax::ast::{Expr,Item};
use syntax::parse::token::*;
use syntax::tokenstream::*;
use syntax::util::small_vector::SmallVector;

use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder; // A trait for expr_usize.
use syntax::ext::quote::rt::Span;
use rustc_plugin::Registry;

#[allow(dead_code)]
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
    })
}


fn parse_expr_pro(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> P<Expr> {

    if args.len() == 0 {
        cx.span_err(sp, "Empty expr ?");
    }

    match &args[0] {
        &TokenTree::Token(_, ref tok) => {
            match tok {
                &Token::BinOp(Or) => {
                    let e = parse_expr(cx,sp,args);
                    return cx.expr_call_ident(sp,cx.ident_of("fnmut2pro"),vec![e])
                }
                &Token::Ident(id) if id.name.as_str() == "move" => {
                    let e = parse_expr(cx,sp,args);
                    return cx.expr_call_ident(sp,cx.ident_of("fnmut2pro"),vec![e])
                }
                _ => {}
            }
        }
        _ => {}
    }

    let mut parser = cx.new_parser_from_tts(args);
    parser.parse_expr().unwrap_or_else(| mut d| {
        d.emit();
        return DummyResult::raw_expr(sp);
    })
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

fn split_on_binop_par(
    cx: &mut ExtCtxt,
    sp: Span,
    args: &[TokenTree],
    ind: usize,
) -> (P<Expr>, P<Expr>) {
    let (s1, s2tmp) = args.split_at(ind);
    let (_, s2) = s2tmp.split_at(1);
    let sp1 = sp.until(args[ind].span());
    let sp2 = args[ind].span().end_point().to(sp.end_point());

    (parse_pro_par(cx, sp1, s1), parse_pro_par(cx, sp2, s2))
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


fn parse_pro_par(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> P<Expr> {

    //print!("parse pro : ");
    //printtts(args);

    if args.len() == 0 {
        cx.expr_ident(sp,cx.ident_of("PNothing"));
    }
    if args.len() == 1 {
        match &args[0] {
            &TokenTree::Token(sp, _) => {
                return parse_expr(cx, sp, args);
            }
            &TokenTree::Delimited(sp,
                                  Delimited {
                                      delim: d,
                                      tts: ref ts,
                                  }) => {
                if d == DelimToken::Paren || d == DelimToken::Brace {
                    return parse_pro_par(cx, sp, &extract_ts(ts.clone().into()));
                } else {
                    cx.span_err(sp, "Process delimited by brackets ?");
                    return DummyResult::raw_expr(sp);
                }
            }
        }
    }
    if args.len() == 2 {
        if let TokenTree::Token(_, Ident(id)) = args[0] {
            if id.name.as_str() == "loop" {
                let n1 = parse_pro_par(cx, args[1].span(), &args[1..2]);
                return cx.expr_method_call(sp, n1, cx.ident_of("ploop_par"), vec![]);
            }
            if id.name.as_str() == "box" {
                let n1 = parse_pro_par(cx, args[1].span(), &args[1..2]);
                return cx.expr_method_call(sp, n1, cx.ident_of("pbox_par"), vec![]);
            }

        }
    }

    if args.len() == 3 {
        if let TokenTree::Token(_, Ident(id)) = args[0] {
            if id.name.as_str() == "choice" {
                let n1 = parse_pro_par(cx, args[1].span(), &args[1..2]);
                let n2 = parse_pro_par(cx, args[2].span(), &args[2..3]);
                return cx.expr_method_call(sp, n1, cx.ident_of("choice_par"), vec![n2]);
            }
            if id.name.as_str() == "present" {
                let n1 = parse_pro_par(cx, args[1].span(), &args[1..2]);
                let n2 = parse_pro_par(cx, args[2].span(), &args[2..3]);
                return cx.expr_method_call(sp, n1, cx.ident_of("present_par"), vec![n2]);
            }
        }
    }


    // reverse for type inference (left associativity)
    for i in (0..args.len()).rev() {
        match args[i] {
            TokenTree::Token(_, ref tok) => {
                match tok {
                    &Token::Semi => {
                        let (p1, p2) = split_on_binop_par(cx, sp, args, i);
                        return cx.expr_method_call(sp, p1, cx.ident_of("seq_par"), vec![p2]);
                    }
                    &Token::OrOr => {
                        let (p1, p2) = split_on_binop_par(cx, sp, args, i);
                        return cx.expr_method_call(sp, p1, cx.ident_of("join_par"), vec![p2]);
                    }

                    _ => {}
                }
            }
            TokenTree::Delimited(_,_) => {}
        }
    }

    parse_expr(cx, sp, args)
}

fn parse_pro(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> P<Expr> {

    // print!("parse pro : ");
    // printtts(args);

    if args.len() == 0 {
        let name = cx.expr_ident(sp,cx.ident_of("nothing"));
        return cx.expr_call(sp,name,vec![]);
    }
    if args.len() == 1 {
        match &args[0] {
            &TokenTree::Token(sp, _) => {
                return parse_expr_pro(cx, sp, args);
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
    if args.len() == 2 {
        if let TokenTree::Token(_, Ident(id)) = args[0] {
            if id.name.as_str() == "loop" {
                let n1 = parse_pro(cx, args[1].span(), &args[1..2]);
                return cx.expr_method_call(sp, n1, cx.ident_of("ploop"), vec![]);
            }
            if id.name.as_str() == "box" {
                let n1 = parse_pro(cx, args[1].span(), &args[1..2]);
                return cx.expr_method_call(sp, n1, cx.ident_of("pbox"), vec![]);
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
            if id.name.as_str() == "present" {
                let n1 = parse_pro(cx, args[1].span(), &args[1..2]);
                let n2 = parse_pro(cx, args[2].span(), &args[2..3]);
                return cx.expr_method_call(sp, n1, cx.ident_of("present"), vec![n2]);
            }
        }
    }


    // reverse for type inference (left associativity)
    for i in (0..args.len()).rev() {
        match args[i] {
            TokenTree::Token(_, ref tok) => {
                match tok {
                    &Token::Semi => {
                        let (p1, p2) = split_on_binop(cx, sp, args, i);
                        return cx.expr_method_call(sp, p1, cx.ident_of("seq"), vec![p2]);
                    }
                    &Token::OrOr => {
                        let (p1, p2) = split_on_binop(cx, sp, args, i);
                        return cx.expr_method_call(sp, p1, cx.ident_of("join"), vec![p2]);
                    }

                    _ => {}
                }
            }
            TokenTree::Delimited(_,_) => {}
        }
    }

    parse_expr_pro(cx, sp, args)
}

fn parse_node(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> P<Expr> {
    if args.len() == 0 {
        cx.expr_ident(sp,cx.ident_of("Nothing"));
    }
    if args.len() == 1 {
        match &args[0] {
            &TokenTree::Token(sp, _) => {
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
            TokenTree::Token(_, ref tok) => {
                match tok {
                    &Token::BinOp(Shr) => {
                        let (p1, p2) = split_on_binop_node(cx, sp, args, i);
                        return cx.expr_method_call(sp, p1, cx.ident_of("nseq"), vec![p2]);
                    }
                    &Token::OrOr => {
                        let (p1, p2) = split_on_binop_node(cx, sp, args, i);
                        return cx.expr_method_call(sp, p1, cx.ident_of("njoin"), vec![p2]);
                    }

                    _ => {}
                }
            }
            TokenTree::Delimited(_,_) => {}
        }
    }

    parse_expr(cx, sp, args)
}

fn expand_pro(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    //print!("\n\n\n\n expand pro :");
    //printtts(args);
    MacEager::expr(parse_pro(cx, sp, args))
}

fn expand_pro_par(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    MacEager::expr(parse_pro_par(cx, sp, args))
}

fn expand_node(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    MacEager::expr(parse_node(cx, sp, args))
}

fn expand_mimpl(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    MacEager::items(parse_mimpl(cx, sp, args))
}


fn parse_mimpl(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> SmallVector<P<Item>>{
    if args.len() < 6 {
        cx.span_err(sp,
                    "mimpl minimal structure : impl for type trait trait_name {...}");
        return SmallVector::new()
    }

    macro_rules! checkkw {
        ($num:tt,$keyword:expr) => {{
            if let TokenTree::Token(_,Ident(id)) = args[$num]{
                if id.name.as_str() != $keyword {
                    cx.span_err(sp, &format!("mimpl: first token must be {}",$keyword));
                    return SmallVector::new();
                }
            }
            else {
                cx.span_err(sp, &format!("mimpl: first token must be {}",$keyword));
                return SmallVector::new();
            }
        }}
    };
    checkkw!(0,"impl");
    let mut forpos : usize = 0;
    let mut traitpos : Vec<usize> = vec![];

    for i in 1..args.len() {
        match &args[i] {
            &TokenTree::Token(spfor, Ident(id)) => {
                if id.name.as_str() == "for" {
                    if forpos != 0 {
                        cx.span_err(spfor, "mimpl: two for keyword" );
                        cx.span_warn(args[forpos].span(), "mimpl: first for here" );
                        return SmallVector::new();
                    }
                    forpos = i;
                }
                if id.name.as_str() == "trait" {
                    traitpos.push(i);
                }
            }
            _ => {}
        }
    }
    if traitpos.is_empty(){
        cx.span_err(sp, "mimpl: no trait");
        return SmallVector::new();
    }
    let mut traitposend : Vec<usize> = traitpos.iter().skip(1).cloned().collect();
    traitposend.push(args.len());
    let mut v = SmallVector::new();
    for (beg,end) in traitpos.iter().zip(traitposend.iter()){
        let mut code = vec![];
        // impl <...>
        for val in &args[0..forpos]{
            code.push(val.clone());
        }
        // trait_name
        for val in &args[beg +1.. end -1]{
            code.push(val.clone());
        }
        // for ... where ...
        for val in &args[forpos..traitpos[0]]{
            code.push(val.clone());
        }
        // {...}
        code.push(args[end-1].clone());

        // println!("code:");
        // printtts(&code);

        let mut parser = cx.new_parser_from_tts(&code);
        match parser.parse_item(){
            Ok(Some(ii)) => {v.push(ii)}
            Ok(None) => {}
            Err(mut d) => {
                d.emit();
                return SmallVector::new();
            }
        }
    }
    return v;
}


#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("pro", expand_pro);
    reg.register_macro("prop", expand_pro_par);
    reg.register_macro("node", expand_node);
    reg.register_macro("mimpl", expand_mimpl);
}
