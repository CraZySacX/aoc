//! Useful utilities
use anyhow::Result;

/// private try from
pub trait PrivateTryFromUsize: Sized {
    /// private try from
    fn private_try_from(n: usize) -> Result<Self>;
}

// impl<T> PrivateTryFromUsize for T
// where
//     T: TryFrom<usize>,
// {
//     #[inline]
//     fn private_try_from(n: usize) -> ::std::result::Result<Self, ()> {
//         T::try_from(n).map_err(|_| ())
//     }
// }

/// no possible bounds violation
macro_rules! try_from_unbounded {
    ($($target:ty),*) => {$(
        impl PrivateTryFromUsize for $target {
            #[inline]
            fn private_try_from(value: usize) -> ::anyhow::Result<Self> {
                Ok(value as $target)
            }
        }
    )*}
}

/// unsigned to signed (only positive bound)
macro_rules! try_from_upper_bounded {
    ($($target:ty),*) => {$(
        impl PrivateTryFromUsize for $target {
            #[inline]
            fn private_try_from(u: usize) -> ::anyhow::Result<$target> {
                if u > (<$target>::MAX as usize) {
                    Err(::anyhow::anyhow!("failed"))
                } else {
                    Ok(u as $target)
                }
            }
        }
    )*}
}

/// ptr_try_from_impls
#[cfg(target_pointer_width = "16")]
mod ptr_try_from_impls {
    use super::PrivateTryFromUsize;

    try_from_unbounded!(u16, u32, u64, u128);
    try_from_unbounded!(i32, i64, i128);
}

/// ptr_try_from_impls
#[cfg(target_pointer_width = "32")]
mod ptr_try_from_impls {
    use super::PrivateTryFromUsize;

    try_from_upper_bounded!(u16);
    try_from_unbounded!(u32, u64, u128);
    try_from_upper_bounded!(i32);
    try_from_unbounded!(i64, i128);
}

/// ptr_try_from_impls
#[cfg(target_pointer_width = "64")]
mod ptr_try_from_impls {
    use super::PrivateTryFromUsize;

    try_from_upper_bounded!(u16, u32);
    try_from_unbounded!(u64, u128);
    try_from_upper_bounded!(i32, i64);
    try_from_unbounded!(i128);
}
