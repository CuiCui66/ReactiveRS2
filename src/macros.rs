
#[macro_export]
macro_rules! pro {

    ( $first:tt >> $($x:tt)>>+ ) => {{
        (pro!($first))
            $(.seq(pro!($x)))*
    }};
    ( $first:tt || $($x:tt)||+ ) => {{
        (pro!($first))
            $(.join(pro!($x)))*

    }};
    ( ( $($x:tt)+ ) ) => {{
        pro!($($x)*)
    }};
    ($x:expr) => {{
        $x
    }}
}

#[macro_export]
macro_rules! node {

    ( $first:tt >> $($x:tt)>>+ ) => {{
        (node!($first))
            $(.nseq(node!($x)))*
    }};
       ( ( $($x:tt)+ ) ) => {{
        node!($($x)*)
    }};
    ($x:expr) => {{
        $x
    }}
}

#[macro_export]
macro_rules! nseq {
    ($first:expr , $($x:expr),+) => {{
        ($first)
            $(.nseq($x))*
    }};
    ( $x:expr) => {$x}
}


#[macro_export]
macro_rules! mrpo {
    ($($x:tt)+ ) => {{
        mp(pro!($($x)*))
    }};
}

#[macro_export]
macro_rules! runtime {
    ($($x:tt)+) => {{
        Runtime::new(mp(pro!($($x)*)))
    }};
}

#[macro_export]
macro_rules! run {
    ($($x:tt)+) => {{
        let mut r = Runtime::new(mp(pro!($($x)*)));
        r.execute();
    }};
}
