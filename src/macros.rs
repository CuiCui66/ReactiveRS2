
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
macro_rules! rtp {
    ($($x:tt)+) => {{
        Runtime::new(mp_par(prop!($($x)*)))
    }};
}

#[macro_export]
macro_rules! run {
    ($($x:tt)+) => {{
        let mut r = Runtime::new(mp(pro!($($x)*)));
        r.execute();
    }};
}

#[macro_export]
macro_rules! runp {
    ($($x:tt)+) => {{
        let mut r = Runtime::new(mp_par(prop!($($x)*)));
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



