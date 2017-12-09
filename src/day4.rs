//! Advent of Code - Day 4 Solution
use clap::{App, Arg, ArgMatches, SubCommand};
use constants::DAY_4;
use error::Result;
use run::AoCYear;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

/// Advent of Code Day 4 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day04")
        .about(DAY_4)
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .required(true)
                .default_value("data/day4/passphrase_list"),
        )
        .arg(
            Arg::with_name("second")
                .short("s")
                .long("second")
                .help("Run the alrgorithm to calculate the value for the 2nd star"),
        )
}

/// Find the solution.
pub fn find_solution(matches: &ArgMatches, _year: &AoCYear) -> Result<u32> {
    let filename = matches.value_of("file").ok_or("Invalid filename!")?;
    let reader = BufReader::new(File::open(filename)?);
    Ok(count_valid_passphrases(
        reader,
        matches.is_present("second"),
    )?)
}

/// Parse the file at `filename` and generate the checksum.
fn count_valid_passphrases<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let mut count = 0;

    for line_result in reader.lines() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        if second_star {
            count += if check_for_anagrams(line)? { 1 } else { 0 };
        } else {
            count += if check_for_duplicates(line)? { 1 } else { 0 };
        }
    }

    Ok(count)
}

/// Check each passphrase for the same word an toss out any that violate.
fn check_for_duplicates(line: &str) -> Result<bool> {
    let words: Vec<&str> = line.split(char::is_whitespace).collect();
    let word_count = words.len();
    let mut word_set = HashSet::new();

    for word in words {
        word_set.insert(word);
    }

    Ok(word_count == word_set.len())
}

/// Check each passphrase for the same anagram and toss out any that violate.
fn check_for_anagrams(line: &str) -> Result<bool> {
    let words: Vec<&str> = line.split(char::is_whitespace).collect();
    let word_count = words.len();
    let mut word_set = HashSet::new();

    for word in words {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort_by(|a, b| b.cmp(a));
        let s = String::from_iter(chars);
        word_set.insert(s);
    }

    Ok(word_count == word_set.len())
}

#[cfg(test)]
mod test {
    use super::{check_for_anagrams, check_for_duplicates};

    #[test]
    fn check_line_for_duplicates() {
        assert!(check_for_duplicates("aa bb cc dd").unwrap_or(false));
        assert!(check_for_duplicates("aa bb cc aaa").unwrap_or(false));
        assert!(!check_for_duplicates("aa bb cc aa").unwrap_or(true));
    }

    #[test]
    fn check_line_for_anagrams() {
        assert!(check_for_anagrams("abcde fghij").unwrap_or(false));
        assert!(!check_for_anagrams("abcde xyz ecdab").unwrap_or(true));
        assert!(check_for_anagrams("a ab abc abd abf abj").unwrap_or(false));
        assert!(check_for_anagrams("iiii oiii ooii oooi oooo").unwrap_or(false));
        assert!(!check_for_anagrams("oiii ioii iioi iiio").unwrap_or(true));
    }
}
