//! Advent of Code - Day 9 Solution
use clap::{App, Arg, SubCommand};
use constants::DAY_9;
use error::Result;
use std::io::BufRead;

/// Advent of Code Day 9 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day09")
        .about(DAY_9)
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .required(true)
                .default_value("data/day9/stream"),
        )
        .arg(
            Arg::with_name("second")
                .short("s")
                .long("second")
                .help("Run the alrgorithm to calculate the value for the 2nd star"),
        )
}

/// Calculate the largest value in a register.
pub fn process_stream<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let mut score = (0, 0);
    for line_result in reader.lines() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        score = process_line_as_chars(line)?;
    }

    if second_star {
        Ok(score.1)
    } else {
        Ok(score.0)
    }
}

/// Process a line as a stream of chars.
fn process_line_as_chars(line: &str) -> Result<(u32, u32)> {
    let mut scores: Vec<u32> = Vec::new();
    let mut current_nesting = 0;
    let mut garbage_count = 0;
    let mut in_garbage = false;
    let mut skip_next = false;

    for c in line.chars() {
        if skip_next {
            skip_next = false;
            continue;
        }
        match c {
            '{' if !in_garbage => {
                current_nesting += 1;
            }
            '}' if !in_garbage => {
                scores.push(current_nesting);
                current_nesting -= 1;
            }
            '<' if !in_garbage => {
                in_garbage = true;
            }
            '!' if in_garbage => {
                skip_next = true;
            }
            '>' if in_garbage => {
                in_garbage = false;
            }
            ',' if current_nesting > 0 && !in_garbage => {
                continue;
            }
            _ if in_garbage => {
                garbage_count += 1;
                continue;
            }
            _ => return Err("Unknown character encountered!".into()),
        }
    }
    Ok((scores.iter().fold(0, |acc, x| acc + x), garbage_count))
}

#[cfg(test)]
mod test {
    use super::process_line_as_chars;

    #[test]
    fn stream_score() {
        assert_eq!(process_line_as_chars("{}").unwrap_or((0, 0)).0, 1);
        assert_eq!(process_line_as_chars("{{{}}}").unwrap_or((0, 0)).0, 6);
        assert_eq!(process_line_as_chars("{{},{}}").unwrap_or((0, 0)).0, 5);
        assert_eq!(
            process_line_as_chars("{{{},{},{{}}}}").unwrap_or((0, 0)).0,
            16
        );
        assert_eq!(
            process_line_as_chars("{<a>,<a>,<a>,<a>}")
                .unwrap_or((0, 0))
                .0,
            1
        );
        assert_eq!(
            process_line_as_chars("{{<ab>},{<ab>},{<ab>},{<ab>}}")
                .unwrap_or((0, 0))
                .0,
            9
        );
        assert_eq!(
            process_line_as_chars("{{<!!>},{<!!>},{<!!>},{<!!>}}")
                .unwrap_or((0, 0))
                .0,
            9
        );
        assert_eq!(
            process_line_as_chars("{{<a!>},{<a!>},{<a!>},{<ab>}}")
                .unwrap_or((0, 0))
                .0,
            3
        )
    }

    #[test]
    fn garbage_score() {
        assert_eq!(process_line_as_chars("<>").unwrap_or((0, 1)).1, 0);
        assert_eq!(
            process_line_as_chars("<random characters>")
                .unwrap_or((0, 0))
                .1,
            17
        );
        assert_eq!(process_line_as_chars("<<<<>").unwrap_or((0, 0)).1, 3);
        assert_eq!(process_line_as_chars("<{!>}>").unwrap_or((0, 0)).1, 2);
        assert_eq!(process_line_as_chars("<!!>").unwrap_or((0, 0)).1, 0);
        assert_eq!(process_line_as_chars("<!!!>>").unwrap_or((0, 0)).1, 0);
        assert_eq!(
            process_line_as_chars("<{o\"i!a,<{i<a>").unwrap_or((0, 0)).1,
            10
        );
    }
}
