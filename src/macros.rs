//! This module and the `promacros` crate try to give syntactic sugar to processes and nodes.
//!
//! # pro!
//!
//! Inside a pro! macro, there are process direct value:
//!
//! * `E` where `E` begins by `|` or `move`: E must have a type implementing `FnMut`:
//!   The macro returns `fnmut2pro(E)`
//! * `once E` : `E` must have a type implementing `FnOnce`:
//!   The macro returns `fnonce2pro(E)`
//! * `val E` :`E` must have a type implementing `FnOnce`:
//!   The macro returns `fnonce2pro(E)`
//! * E in any other case : `E` must be a rust expression whose type implements `Process` :
//!   The macro returns `E`.
//!
//! All of these expressions must not contain any `;` or `||` directly i.e
//! not encapsulate in `()`, `[]` or `{}`.
//! from the basic expression we build the grammar with :
//!
//! * `P ; Q`: transformed to `P.seq(Q)`
//! * `P || Q`: transformed to `P.join(Q)`
//! * `choice {P}{Q}`: transformed to `P.choice(Q)`
//! * `present {P}{Q}`: transformed to `P.present(Q)`
//! * `loop {P}`: transformed to `P.ploop()`
//!
//! `once`, `val`, `choice` and `present` are considered as keyword in the macro `pro!` and thus
//! should not be used if not encapsulated in `()`, `[]` or `{}`.
//!
//! A `;` with nothing behind add a PNothing and thus force the output type to be ();
//!
//! # node!
//!
//! The node! macro allow only expressions to be directly a node, the constructions are:
//!
//! * `P >> Q`: transformed to `P.nseq(Q)`
//! * `P || Q`: transformed to `P.njoin(Q)`
//! * `choice {P}{Q}`: transformed to `P.alter(Q)`
//!
//! # mimpl!
//!
//! In order to implement multiple traits of the form:
//! `impl<T> A for B where C {D}` and
//! `impl<T> A2 for B where C {D2}`, one can write
//!
//! `mimpl!{impl<T> for B where C trait A {D} trait A2 {D2}}`
//!
//!

/// Creates a runtime from a process description in terms of pro! macro syntax
#[macro_export]
macro_rules! rt {
    ($($x:tt)+) => {{
        Runtime::new(pro!($($x)*))
    }};
}

/// Creates a runtime from a process description in terms of pro! macro syntax and run it
#[macro_export]
macro_rules! run {
    ($($x:tt)+) => {{
        let mut r = Runtime::new(pro!($($x)*));
        r.execute();
    }};
}

/// `nodei!(x)` is equivalent to `node!((x) >> Ignore{})`
#[macro_export]
macro_rules! nodei {
    ($($x:tt)+ ) => {{
        node!(($($x)*) >> Ignore {})
    }};
}

/// `nodep!(x)` is equivalent to `node!(GenP{} >> x)`
#[macro_export]
macro_rules! nodep {
    ($($x:tt)+ ) => {{
        node!(GenP {} >> ($($x)*))
    }};
}

/// `nodepi!(x)` is equivalent to `node!(GenP{} >> x >> Ignore {})`
#[macro_export]
macro_rules! nodepi {
    ($($x:tt)+ ) => {{
        node!(GenP{} >> ($($x)*) >> Ignore{})
    }};
}

/// Genrated Boxed associated type implementation for ToBoxedProcess for non-immediate processes
#[macro_export]
macro_rules! boxed_ni {
    ($in:ty) => (
        type Boxed = ProcessNotIm<
            'a,
        $in,
        <Self as IntProcess<'a, $in>>::Out,
        <Self as IntProcess<'a, $in>>::MarkOnce,
        <Self as IntProcessNotIm<'a, $in>>::NI,
        <Self as IntProcessNotIm<'a, $in>>::NO,
        >;
    );
}

/// Genrated tobox implementation for ToBoxedProcess for non-immediate processes
#[macro_export]
macro_rules! tobox_ni {
    () => (
        fn tobox(self) -> Self::Boxed {
            ProcessNotIm(box self)
        }
    );
}

/// Genrated Boxed associated type implementation for ToBoxedProcess for immediate processes
#[macro_export]
macro_rules! boxed_i {
    ($in:ty) => (
        type Boxed = ProcessIm<
            'a,
        $in,
        <Self as IntProcess<'a, $in>>::Out,
        <Self as IntProcess<'a, $in>>::MarkOnce,
        <Self as IntProcessIm<'a, $in>>::NIO,
        >;
    );
}

/// Genrated tobox implementation for ToBoxedProcess for immediate processes
#[macro_export]
macro_rules! tobox_i {
    () => (
        fn tobox(self) -> Self::Boxed {
            ProcessIm(box self)
        }
    );
}

/// take an mimpl syntax impl item and add an implementation of ToBoxedProcess for immediate
#[macro_export]
macro_rules! implIm {
    ($in:ty,$($x:tt)+) => (
        mimpl!(
            $($x)*
            trait ToBoxedProcess<'a, $in>
            {
                boxed_i!($in);
                tobox_i!();
            }
        );
    );
}

/// take an mimpl syntax impl item and add an implementation of ToBoxedProcess for non-immediate
#[macro_export]
macro_rules! implNI {
    ($in:ty,$($x:tt)+) => (
        mimpl!(
            $($x)*
            trait ToBoxedProcess<'a, $in>
            {
                boxed_ni!($in);
                tobox_ni!();
            }
        );
    );
}


