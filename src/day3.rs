//! Advent of Code - Day 3 Solution
use error::Result;
use std::collections::HashMap;
use std::convert::TryFrom;

/// Calculate the number of steps it will take to drain the given
/// value out of the (0,0) port.
pub fn calculate_steps(value: u32) -> Result<i32> {
    let final_tuple = calculate_tuple(value)?;
    manhattan_distance(final_tuple, (0, 0))
}

/// Calculate (x,y) tuple for a given value.
fn calculate_tuple(value: u32) -> Result<(i32, i32)> {
    let mut current_tuple: (i32, i32) = (0, 0);
    let mut generated = 1;

    for shell in 0.. {
        let upper_limit = generated + (8 * shell);
        if value <= upper_limit {
            let needed = value - generated;
            if needed > 0 {
                generate_next_n_tuples(&mut current_tuple, shell, value - generated)?;
            }
            break;
        } else {
            let ty_shell: i32 = TryFrom::try_from(shell)?;
            // We know the last tuple in any shell `x` is `(x, -x)`, so we just set it.
            current_tuple = (ty_shell, -ty_shell);
            // Bump the upper limit
            generated = upper_limit;
        }
    }
    Ok(current_tuple)
}

/// Calculate the manhattan distance between two (x,y) tuples.
fn manhattan_distance(from: (i32, i32), to: (i32, i32)) -> Result<i32> {
    Ok((from.0 - to.0).abs() + (from.1 - to.1).abs())
}

/// Calculate the last tuple in the given shell
fn generate_next_n_tuples(start_tuple: &mut (i32, i32), shell: u32, count: u32) -> Result<()> {
    start_tuple.0 += 1;
    let side_length = (8 * shell) / 4;
    let max_y: i32 = TryFrom::try_from(shell)?;
    let min_x: i32 = -TryFrom::try_from(shell)?;
    let min_y: i32 = -TryFrom::try_from(shell)?;

    for idx in 1..count {
        if start_tuple.1 < max_y && idx <= side_length {
            start_tuple.1 += 1;
        } else if start_tuple.0 > min_x && idx <= side_length * 2 {
            start_tuple.0 -= 1;
        } else if start_tuple.1 > min_y && idx <= side_length * 3 {
            start_tuple.1 -= 1;
        } else {
            start_tuple.0 += 1;
        }
    }

    Ok(())
}

fn tuple_map(_max_value: u32) -> Result<u32> {
    let mut tuple_map: HashMap<(i32, i32), u32> = HashMap::new();
    tuple_map.insert((0,0), 1);
    
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::calculate_steps;

    #[test]
    fn steps() {
        assert_eq!(calculate_steps(1).unwrap_or(-1), 0);
        assert_eq!(calculate_steps(12).unwrap_or(-1), 3);
        assert_eq!(calculate_steps(23).unwrap_or(-1), 2);
        assert_eq!(calculate_steps(1024).unwrap_or(-1), 31);
    }
}
