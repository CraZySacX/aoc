//! Useful utilities
use error::{Error, Result};
use std::convert::TryFrom;
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
            _ => Err("Invalid prefix!".into()),
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
