
#[macro_export]
macro_rules! pro {

    ( $first:tt >> $($x:tt)>>+ ) => {{
        ($first)
            $(.seq($x))*
    }};
    ( $first:tt || $($x:tt)||+ ) => {{
        ($first)
            $(.join($x))*

    }};
    ( ( $($x:tt)+ ) ) => {{
        pro!($($x)*)
    }};
    ($x:expr) => {{
        $x
    }}
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
