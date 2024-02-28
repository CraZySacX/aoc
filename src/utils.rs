//! Useful utilities
use anyhow::{anyhow, Error, Result};
use std::time::Duration;

/// Nanoseconds per nanosecond
const NANOSECONDS_PER_NANOSECOND: f64 = 1.;
/// Microseconds per nanosecond
const NANOSECONDS_PER_MICROSECOND: f64 = 1_000.;
/// Milliseconds per nanosecond
const NANOSECONDS_PER_MILLISECOND: f64 = 1_000_000.;
/// Nanoseconds per second
const NANOSECONDS_PER_SECOND: f64 = 1_000_000_000.;
/// Microseconds per second
const MICROSECONDS_PER_SECOND: f64 = 1_000_000.;
/// Milliseconds per second
const MILLISECONDS_PER_SECOND: f64 = 1_000.;
/// Seconds per second
const SECONDS_PER_SECOND: f64 = 1.;

/// Output Factor
pub enum Prefix {
    /// Nanoseconds
    Nanos,
    /// Microseconds
    Micros,
    /// Milliseconds
    Millis,
    /// Seconds
    Seconds,
}

impl<'a> TryFrom<&'a str> for Prefix {
    type Error = Error;
    fn try_from(prefix: &str) -> Result<Self> {
        match prefix {
            "ns" => Ok(Prefix::Nanos),
            "us" => Ok(Prefix::Micros),
            "ms" => Ok(Prefix::Millis),
            "s" => Ok(Prefix::Seconds),
            _ => Err(anyhow!("Invalid prefix!")),
        }
    }
}

/// Convert a `Duration` to nanoseconds.
pub fn as_ns(duration: &Duration) -> Result<f64> {
    as_x(duration, &Prefix::Nanos)
}

/// Convert a `Duration` to microseconds.
pub fn as_us(duration: &Duration) -> Result<f64> {
    as_x(duration, &Prefix::Micros)
}

/// Convert a `Duration` to milliseconds.
pub fn as_ms(duration: &Duration) -> Result<f64> {
    as_x(duration, &Prefix::Millis)
}

/// Convert a `Duration` to seconds.
pub fn as_s(duration: &Duration) -> Result<f64> {
    as_x(duration, &Prefix::Seconds)
}

/// Convert a `Duration` to the given prefix.
fn as_x(duration: &Duration, output: &Prefix) -> Result<f64> {
    let (nanos_factor, secs_factor) = match *output {
        Prefix::Nanos => (NANOSECONDS_PER_NANOSECOND, NANOSECONDS_PER_SECOND),
        Prefix::Micros => (NANOSECONDS_PER_MICROSECOND, MICROSECONDS_PER_SECOND),
        Prefix::Millis => (NANOSECONDS_PER_MILLISECOND, MILLISECONDS_PER_SECOND),
        Prefix::Seconds => (NANOSECONDS_PER_SECOND, SECONDS_PER_SECOND),
    };

    let converted_nanos = f64::from(duration.subsec_nanos()) / nanos_factor;
    let secs_as_signed: u32 = TryFrom::try_from(duration.as_secs())?;
    let converted_secs: f64 = f64::from(secs_as_signed) * secs_factor;
    Ok(converted_secs + converted_nanos)
}

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
                if u > (<$target>::max_value() as usize) {
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
