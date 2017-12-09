//! Advent of Code - Day 9 Solution
use error::Result;
use std::io::BufRead;

/// Calculate the largest value in a register.
pub fn process_stream<T: BufRead>(reader: T, _second_star: bool) -> Result<u32> {
    let mut score = 0;
    for line_result in reader.lines() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        score = process_line_as_chars(line)?;
    }
    Ok(score)
}

/// Process a line as a stream of chars.
fn process_line_as_chars(line: &str) -> Result<u32> {
    let mut scores: Vec<u32> = Vec::new();
    let mut current_nesting = 0;
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
                continue;
            }
            _ => return Err("Unknown character encountered!".into()),
        }
    }
    Ok(scores.iter().fold(0, |acc, x| acc + x))
}

#[cfg(test)]
mod test {
    use super::process_line_as_chars;

    #[test]
    fn simple_streams() {
        assert_eq!(process_line_as_chars("{}").unwrap_or(0), 1);
        assert_eq!(process_line_as_chars("{{{}}}").unwrap_or(0), 6);
        assert_eq!(process_line_as_chars("{{},{}}").unwrap_or(0), 5);
        assert_eq!(process_line_as_chars("{{{},{},{{}}}}").unwrap_or(0), 16);
        assert_eq!(process_line_as_chars("{<a>,<a>,<a>,<a>}").unwrap_or(0), 1);
        assert_eq!(
            process_line_as_chars("{{<ab>},{<ab>},{<ab>},{<ab>}}").unwrap_or(0),
            9
        );
        assert_eq!(
            process_line_as_chars("{{<!!>},{<!!>},{<!!>},{<!!>}}").unwrap_or(0),
            9
        );
        assert_eq!(
            process_line_as_chars("{{<a!>},{<a!>},{<a!>},{<ab>}}").unwrap_or(0),
            3
        )
    }
}
