//! Advent of Code - Day 6 Solution
use clap::{App, Arg, ArgMatches, SubCommand};
use constants::DAY_6;
use error::Result;
use run::AoCYear;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Advent of Code Day 6 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day06")
        .about(DAY_6)
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .required(true)
                .default_value("data/day6/blocks_list"),
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
    Ok(reallocations_until_match(
        reader,
        matches.is_present("second"),
    )?)
}

/// Parse the file at `filename` and generate the checksum.
fn reallocations_until_match<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let mut reallocations = 0;
    for line_result in reader.lines() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        reallocations = reallocate_memory(line, second_star)?;
    }

    Ok(reallocations)
}

/// Reallocate some memory blocks
fn reallocate_memory(line: &str, find_again: bool) -> Result<u32> {
    // Convert the line to a vector of u32.
    let vals_iter = line.split_whitespace();
    let mut vals_vec = Vec::new();
    for val_str in vals_iter {
        vals_vec.push(val_str.parse::<u32>()?);
    }

    // Setup some state.
    let len = vals_vec.len();
    let mut once_more = find_again;
    let mut steps = 1;
    let mut seen = HashSet::new();

    loop {
        // We've seen the current vec, so put into set.
        seen.insert(vals_vec.clone());

        // Find the max and first position of max.
        let max_vec = vals_vec.clone();
        let pos_vec = vals_vec.clone();
        let max = max_vec.iter().max().ok_or("Unable to find max")?;
        let pos = pos_vec
            .iter()
            .position(|&x| x == *max)
            .ok_or("Unable to find pos of max")?;

        // Reset the max to 0
        vals_vec[pos] = 0;

        // Cycle through the vec, reallocating
        for i in 0..*max {
            let idx = (pos + (i + 1) as usize) % len;
            vals_vec[idx] += 1;
        }

        // Check if we have seen the resulting vec
        if seen.get(&vals_vec).is_some() {
            // If we have, but we want to find the next occurence
            // then reset some state and continue.
            if once_more {
                steps = 1;
                seen.clear();
                once_more = false;
                continue;
            }
            // Otherwise we are done.
            break;
        } else {
            // If we haven't, increment the step count and loop
            steps += 1;
        }
    }
    Ok(steps)
}

#[cfg(test)]
mod test {
    use super::reallocate_memory;

    #[test]
    fn reallocate() {
        assert_eq!(reallocate_memory("0 2 7 0", false).unwrap_or(0), 5);
        assert_eq!(reallocate_memory("0 2 7 0", true).unwrap_or(0), 4);
    }
}
