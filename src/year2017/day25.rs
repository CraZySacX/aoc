//! Advent of Code - Day 25 Solution
use bytecount;
use error::{Error, Result};
use regex::Regex;
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::fmt;
use std::io::BufRead;
use utils::PrivateTryFromUsize;

/// The direction to move on the tape
#[derive(Debug, Default)]
enum Move {
    #[default]
    /// Left
    Left,
    /// Right
    Right,
}

impl<'a> TryFrom<&'a str> for Move {
    type Error = Error;
    fn try_from(val: &str) -> Result<Self> {
        Ok(match val {
            "left" => Move::Left,
            "right" => Move::Right,
            _ => return Err(format!("Invalid move value: {val}").into()),
        })
    }
}

impl fmt::Display for Move {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "{}",
            match *self {
                Move::Left => "left",
                Move::Right => "right",
            }
        )
    }
}

/// A state definition.
#[derive(Debug, Default)]
struct State {
    /// What to write if the current slot is 0.
    zero_write: u8,
    /// Where to move if the current slot is 0.
    zero_move: Move,
    /// What state is next if the current slot is 0.
    zero_next: char,
    /// What to write if the current slot is 1.
    one_write: u8,
    /// Where to move if the current slot is 1.
    one_move: Move,
    /// What state is next if the current slot is 1.
    one_next: char,
}

impl fmt::Display for State {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "State {{ ")?;
        write!(fmt, "zero_write: {}, ", self.zero_write)?;
        write!(fmt, "zero_move: {}, ", self.zero_move)?;
        write!(fmt, "zero_next: {}, ", self.zero_next)?;
        write!(fmt, "one_write: {}, ", self.one_write)?;
        write!(fmt, "one_move: {}, ", self.one_move)?;
        write!(fmt, "one_next: {} }}", self.one_next)
    }
}

/// Find the solution for Advent of Code 2017
pub fn find_solution<T: BufRead>(reader: T, _second_star: bool) -> Result<u32> {
    let mut tape: Vec<u8> = vec![0; 10_000_000];
    let mut states: BTreeMap<char, State> = BTreeMap::new();

    let begin_re = Regex::new(r"^Begin in state ([A-Z])\.$")?;
    let dc_re = Regex::new(r"^Perform a diagnostic checksum after (\d+) steps\.$")?;
    let in_state_re = Regex::new(r"^In state ([A-Z]):$")?;
    let if_curr_re = Regex::new(r"If the current value is (\d+):$")?;
    let write_val_re = Regex::new(r" - Write the value (\d+)\.$")?;
    let move_re = Regex::new(r"- Move one slot to the (right|left)\.$")?;
    let cont_re = Regex::new(r"- Continue with state ([A-Z])\.$")?;

    let mut start_state = 'A';
    let mut step_count = 0;
    let mut parsing_state = false;
    let mut curr_state = 'A';
    let mut curr_val = 0;

    for line in reader.lines().filter_map(|x| x.ok()) {
        if begin_re.is_match(&line) {
            let caps = begin_re.captures(&line).ok_or("invalid begin captures")?;
            let state_str = caps.get(1).ok_or("invalid state value")?.as_str();
            let val = state_str.parse::<char>()?;
            start_state = val;
        } else if dc_re.is_match(&line) {
            let caps = dc_re.captures(&line).ok_or("invalid diagnostic checksum captures")?;
            let steps_str = caps.get(1).ok_or("invalid diagnostic checksum value")?.as_str();
            let steps = steps_str.parse::<usize>()?;
            step_count = steps;
        } else if in_state_re.is_match(&line) {
            let caps = in_state_re.captures(&line).ok_or("invalid in state captures")?;
            let state_str = caps.get(1).ok_or("invalid in state value")?.as_str();
            let val = state_str.parse::<char>()?;
            parsing_state = true;
            curr_state = val;
            states.insert(val, Default::default());
        } else if if_curr_re.is_match(&line) && parsing_state {
            let caps = if_curr_re.captures(&line).ok_or("invalid if current value captures")?;
            let val_str = caps.get(1).ok_or("invalid if current value")?.as_str();
            let val = val_str.parse::<u8>()?;
            curr_val = val;
        } else if write_val_re.is_match(&line) && parsing_state {
            let caps = write_val_re.captures(&line).ok_or("invalid write value captures")?;
            let val_str = caps.get(1).ok_or("invalid write value")?.as_str();
            let val = val_str.parse::<u8>()?;
            let state_ptr = states.entry(curr_state).or_insert_with(Default::default);

            if curr_val == 0 {
                state_ptr.zero_write = val;
            } else if curr_val == 1 {
                state_ptr.one_write = val;
            } else {
                return Err("Invalid curr value".into());
            }
        } else if move_re.is_match(&line) {
            let caps = move_re.captures(&line).ok_or("invalid move captures")?;
            let move_str = caps.get(1).ok_or("invalid move value")?.as_str();
            let state_ptr = states.entry(curr_state).or_insert_with(Default::default);

            if curr_val == 0 {
                state_ptr.zero_move = TryFrom::try_from(move_str)?;
            } else if curr_val == 1 {
                state_ptr.one_move = TryFrom::try_from(move_str)?;
            } else {
                return Err("Invalid curr value".into());
            }
        } else if cont_re.is_match(&line) {
            let caps = cont_re.captures(&line).ok_or("invalid continue captures")?;
            let cont_str = caps.get(1).ok_or("invalid continue value")?.as_str();
            let val = cont_str.parse::<char>()?;
            let state_ptr = states.entry(curr_state).or_insert_with(Default::default);

            if curr_val == 0 {
                state_ptr.zero_next = val;
            } else if curr_val == 1 {
                state_ptr.one_next = val;
            } else {
                return Err("Invalid curr value".into());
            }
        } else if line.is_empty() && parsing_state {
            parsing_state = false;
        } else if line.is_empty() {
            // Do nothing.
        } else {
            return Err(format!("Unable to parse line: {line}").into());
        }
    }

    let mut curr_idx = 5_000_000;
    curr_state = start_state;

    for _ in 0..step_count {
        let tape_val = tape.get_mut(curr_idx).ok_or("invalid tape value")?;
        let state = states.get(&curr_state).ok_or("invalid state value")?;

        if *tape_val == 0 {
            *tape_val = state.zero_write;
            match state.zero_move {
                Move::Left => curr_idx -= 1,
                Move::Right => curr_idx += 1,
            }
            curr_state = state.zero_next;
        } else {
            *tape_val = state.one_write;
            match state.one_move {
                Move::Left => curr_idx -= 1,
                Move::Right => curr_idx += 1,
            }
            curr_state = state.one_next;
        }
    }

    let count = bytecount::count(&tape, 1);
    u32::private_try_from(count)
}

#[cfg(test)]
mod one_star {
    #[test]
    fn solution() {}
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {}
}
