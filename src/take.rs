use std::mem;

/// C++ std::move like function
pub(crate) fn take<T>(t: &mut T) -> T
where
    T: Default,
{
    let mut res = T::default();
    mem::swap(&mut res, t);
    res
}
