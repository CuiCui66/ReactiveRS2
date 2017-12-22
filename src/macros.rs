
#[macro_export]
macro_rules! rt {
    ($($x:tt)+) => {{
        Runtime::new(pro!($($x)*))
    }};
}

#[macro_export]
macro_rules! run {
    ($($x:tt)+) => {{
        let mut r = Runtime::new(pro!($($x)*));
        r.execute();
    }};
}

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



