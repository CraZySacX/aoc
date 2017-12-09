//! Advent of Code - Day 1 Solution
use clap::{App, Arg, ArgMatches, SubCommand};
use constants::DAY_1;
use error::Result;

/// The Advent of Code Day 1 `SubCommand`.
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day01")
        .about(DAY_1)
        .arg(
            Arg::with_name("second")
                .short("s")
                .long("second")
                .help("Run the alrgorithm to calculate the value for the 2nd star"),
        )
        .arg(Arg::with_name("value").required(true))
}

/// Find the solution.
pub fn find_solution(matches: &ArgMatches) -> Result<u32> {
    let value = matches
        .value_of("value")
        .ok_or("This should never happen due to clap validation!")?;

    Ok(val(value, matches.is_present("second"))?)
}

/// Calculate the 'inverse captcha' value for a byte array.
fn val(input: &str, lookahead: bool) -> Result<u32> {
    let byte_arr = input.as_bytes();
    let len = byte_arr.len();
    let la_idx = if lookahead { len / 2 } else { 1 };

    let mut sum = 0;

    for (idx, curr) in byte_arr.iter().enumerate() {
        let next_idx = (idx + la_idx) % len;

        if *curr == byte_arr[next_idx] {
            sum += u32::from(byte_arr[idx]) - 48;
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod test {
    use super::val;

    #[test]
    fn default_lookahead_of_1() {
        assert_eq!(val("1221", false).unwrap_or(0), 3);
        assert_eq!(val("1111", false).unwrap_or(0), 4);
        assert_eq!(val("1234", false).unwrap_or(0), 0);
        assert_eq!(val("91212129", false).unwrap_or(0), 9);
    }

    #[test]
    fn lookahead_of_half_len() {
        assert_eq!(val("1212", true).unwrap_or(0), 6);
        assert_eq!(val("1221", true).unwrap_or(0), 0);
        assert_eq!(val("123425", true).unwrap_or(0), 4);
        assert_eq!(val("123123", true).unwrap_or(0), 12);
    }
}
