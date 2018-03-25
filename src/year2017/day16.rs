//! Advent of Code - Day 16 Solution
use error::Result;
use regex::Regex;
use std::io::BufRead;

/// Various Dance Moves
enum Move {
    /// Exchange pos1 with pos2
    Exchange(u8, u8),
    /// Rotate x from end to beginning maintaining order
    Spin(u32),
    /// Swap name1 with name2
    Partner(char, char),
}

/// Find the solution for Advent of Code 2017
pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    use std::io::{self, Write};
    let mut moves: Vec<Move> = Vec::new();
    let mut dancers = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'];
    let orig = dancers.clone();

    for line_result in reader.lines() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        generate_moves(line, &mut moves)?;
    }

    if second_star {
        // The trick here is the patten repeats.  So we only need
        // to calculate 1_000_000_000 % repeat to figure this one out.
        let mut repeat = 0;
        for i in 0..1_000_000_000 {
            apply_moves(&moves, &mut dancers)?;
            if dancers == orig {
                repeat = i + 1;
                break;
            }
        }
        for _ in 0..(1_000_000_000 % repeat) {
            apply_moves(&moves, &mut dancers)?;
        }
    } else {
        apply_moves(&moves, &mut dancers)?;
    }
    for i in dancers {
        write!(io::stdout(), "{}", i)?;
    }
    writeln!(io::stdout(), "")?;
    Ok(0)
}

/// Generates the moves vector
fn generate_moves(line: &str, moves: &mut Vec<Move>) -> Result<()> {
    let tokens: Vec<&str> = line.split(',').collect();
    let spin_re = Regex::new(r"s(\d{1,2})")?;
    let exchange_re = Regex::new(r"x(\d{1,2})/(\d{1,2})")?;
    let partner_re = Regex::new(r"p([a-p])/([a-p])")?;

    for token in tokens {
        if spin_re.is_match(token) {
            let caps = spin_re.captures(token).ok_or("invalid spin captures")?;
            let val_str = caps.get(1).ok_or("invalid spin value")?.as_str();
            let val = val_str.parse::<u32>()?;
            moves.push(Move::Spin(val));
        } else if exchange_re.is_match(token) {
            let caps = exchange_re.captures(token).ok_or("invalid exchange captures")?;
            let pos1_str = caps.get(1).ok_or("invalid exchange pos1")?.as_str();
            let pos2_str = caps.get(2).ok_or("invalid exchange pos2")?.as_str();
            let pos1 = pos1_str.parse::<u8>()?;
            let pos2 = pos2_str.parse::<u8>()?;
            moves.push(Move::Exchange(pos1, pos2));
        } else if partner_re.is_match(token) {
            let caps = partner_re.captures(token).ok_or("invalid partner captures")?;
            let name1_str = caps.get(1).ok_or("invalid partner name1")?.as_str();
            let name2_str = caps.get(2).ok_or("invalid partner name2")?.as_str();
            let name1 = name1_str.chars().nth(0).ok_or("name1 not a char")?;
            let name2 = name2_str.chars().nth(0).ok_or("name2 not a char")?;
            moves.push(Move::Partner(name1, name2));
        } else {
            let no_match_found = format!("Invalid token: {}", token);
            return Err(no_match_found.into());
        }
    }
    Ok(())
}

/// Apply moves
fn apply_moves(moves: &[Move], dancers: &mut [char]) -> Result<()> {
    for mov in moves {
        match *mov {
            Move::Spin(ref x) => {
                let len = dancers.len();
                dancers.rotate_left(len - *x as usize);
            }
            Move::Exchange(ref x, ref y) => {
                let first = dancers[*x as usize];
                let second = dancers[*y as usize];
                dancers[*y as usize] = first;
                dancers[*x as usize] = second;
            }
            Move::Partner(ref x, ref y) => {
                let mut idx_x = 0;
                let mut idx_y = 0;
                let mut found = (false, false);

                for (i, val) in dancers.iter().enumerate() {
                    if found == (true, true) {
                        break;
                    }
                    if val == x {
                        idx_x = i;
                        found.0 = true;
                        continue;
                    }
                    if val == y {
                        idx_y = i;
                        found.1 = true;
                        continue;
                    }
                }

                dancers[idx_y] = *x;
                dancers[idx_x] = *y;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod one_star {
    #[test]
    fn solution() {
        let mut dancers = vec!['a', 'b', 'c', 'd', 'e'];
        let mut moves = Vec::new();
        super::generate_moves("s1,x3/4,pe/b", &mut moves).expect("");
        assert_eq!(moves.len(), 3);
        super::apply_moves(&moves, &mut dancers).expect("");
        assert_eq!(dancers, vec!['b', 'a', 'e', 'd', 'c']);
        moves.clear();
        dancers = vec!['a', 'b', 'c', 'd', 'e'];
        super::generate_moves("s3", &mut moves).expect("");
        super::apply_moves(&moves, &mut dancers).expect("");
        assert_eq!(dancers, vec!['c', 'd', 'e', 'a', 'b']);

        moves.clear();
        dancers = vec!['a', 'b', 'c', 'd', 'e'];
        super::generate_moves("x0/3", &mut moves).expect("");
        super::apply_moves(&moves, &mut dancers).expect("");
        assert_eq!(dancers, vec!['d', 'b', 'c', 'a', 'e']);
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {
        assert!(true);
    }
}
