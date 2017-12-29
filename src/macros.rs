
#[macro_export]
macro_rules! rt {
    ($($x:tt)+) => {{
        Runtime::new(pro!($($x)*))
    }};
}

// #[macro_export]
// macro_rules! rtp {
//     ($($x:tt)+) => {{
//         RuntimePar::new(mp_par(prop!($($x)*)))
//     }};
// }

#[macro_export]
macro_rules! run {
    ($($x:tt)+) => {{
        let mut r = Runtime::new(pro!($($x)*));
        r.execute();
    }};
}

// #[macro_export]
// macro_rules! runp {
//     ($($x:tt)+) => {{
//         let mut r = RuntimePar::new(mp_par(prop!($($x)*)));
//         r.execute();
//     }};
// }

#[macro_export]
macro_rules! nodei {
    ($($x:tt)+ ) => {{
        node!(($($x)*) >> Ignore)
    }};
}

#[macro_export]
macro_rules! nodep {
    ($($x:tt)+ ) => {{
        node!(GenP >> ($($x)*))
    }};
}

#[macro_export]
macro_rules! nodepi {
    ($($x:tt)+ ) => {{
        node!(GenP >> ($($x)*) >> Ignore)
    }};
}

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

#[macro_export]
macro_rules! tobox_ni {
    () => (
        fn tobox(self) -> Self::Boxed {
            ProcessNotIm(box self)
        }
    );
}

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

#[macro_export]
macro_rules! tobox_i {
    () => (
        fn tobox(self) -> Self::Boxed {
            ProcessIm(box self)
        }
    );
}

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


