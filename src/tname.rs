pub use std::intrinsics::type_name;

pub fn tname<T : ?Sized>() -> String{
    let s = String::from(unsafe { type_name::<T>() });
    s.replace("ReactiveRS2::node::ChoiceData","CD")
        .replace("ReactiveRS2::node::","")
        .replace("<","\\<")
        .replace(">","\\>")
}
