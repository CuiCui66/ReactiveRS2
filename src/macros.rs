
#[macro_export]
macro_rules! mpro {
    ($($x:tt)+ ) => {{
        mp(pro!($($x)*))
    }};
}

#[macro_export]
macro_rules! rt {
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
