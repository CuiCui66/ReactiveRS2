//! crate for small reusable function
//! unsafe code should be here or behind a funsafe gate


//  _____  _    _  _______
// |_   _|/ \  | |/ / ____|
//   | | / _ \ | ' /|  _|
//   | |/ ___ \| . \| |___
//   |_/_/   \_\_|\_\_____|


use std::mem;

/// C++ std::move like function extract a value from a reference that is about to die
/// The reference is filled with T::default()
pub(crate) fn take<T>(t: &mut T) -> T
    where
    T: Default,
{
    let mut res = T::default();
    mem::swap(&mut res, t);
    res
}

//  _____ _   _    _    __  __ _____
// |_   _| \ | |  / \  |  \/  | ____|
//   | | |  \| | / _ \ | |\/| |  _|
//   | | | |\  |/ ___ \| |  | | |___
//   |_| |_| \_/_/   \_\_|  |_|_____|

pub use std::intrinsics::type_name;

/// return the name of type with few modification relevant to this library
pub fn tname<T : ?Sized>() -> String{
    let s = String::from(unsafe { type_name::<T>() });
    s.replace("ReactiveRS2::node::ChoiceData","CD")
        .replace("ReactiveRS2::node::","")
        .replace("<","\\<")
        .replace(">","\\>")
}

// __     __        ____     _
// \ \   / /__  ___|___ \   / \   _ __ _ __ __ _ _   _
//  \ \ / / _ \/ __| __) | / _ \ | '__| '__/ _` | | | |
//   \ V /  __/ (__ / __/ / ___ \| |  | | | (_| | |_| |
//    \_/ \___|\___|_____/_/   \_\_|  |_|  \__,_|\__, |
//                                               |___/


use std::ptr;

#[macro_export]
macro_rules! vec2array {
    ($vec:expr,$num:expr) => {{
        #[inline(always)]
        fn aux<T>(v:Vec<T>) -> [T; $num]{
            unsafe {
                let mut res : [T; $num] = mem::uninitialized();
                for (i,val) in v.into_iter().enumerate(){
                    ptr::write(&mut res[i], val);
                }
                res
            }
        };
        assert_eq!($vec.len(),$num);
        aux($vec)
    }};
}

//  ______        ___    ____ _____
// / ___\ \      / / \  |  _ \___ /
// \___ \\ \ /\ / / _ \ | |_) ||_ \
//  ___) |\ V  V / ___ \|  __/___) |
// |____/  \_/\_/_/   \_\_|  |____/


// left circular permutation
#[allow(unused)]
pub fn swap3<T>(x: &mut T, y: &mut T, z: &mut T){
    // from https://doc.rust-lang.org/std/ptr/fn.copy_nonoverlapping.html.
    unsafe {
        let mut t: T = mem::uninitialized();

        // Perform the swap, `&mut` pointers never alias
        ptr::copy_nonoverlapping(x, &mut t, 1);
        ptr::copy_nonoverlapping(y, x, 1);
        ptr::copy_nonoverlapping(z, y, 1);
        ptr::copy_nonoverlapping(&t, z, 1);

        // z and t now point to the same thing, but we need to completely forget `t`
        // because it's no longer relevant.
        mem::forget(t);
    }
}

//   ____ ____  _   _
//  / ___|  _ \| | | |_ __   __ _ _   _ ___  ___
// | |   | |_) | | | | '_ \ / _` | | | / __|/ _ \
// | |___|  __/| |_| | |_) | (_| | |_| \__ \  __/
//  \____|_|    \___/| .__/ \__,_|\__,_|___/\___|
//                   |_|

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[allow(unused)]
pub fn cpu_pause(){
    unsafe {
        asm!("pause");
    }
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub fn cpu_pause(){
}

