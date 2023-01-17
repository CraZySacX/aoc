//! Advent of Code - Day 23 'Coprocessor Conflagration' Solution
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::io::BufRead;

/// A value can either be a pointer to a register or a number.
#[derive(Debug, Eq, PartialEq)]
enum Value {
    /// A number value.
    Number(i64),
    /// A registe pointer value.
    Register(String),
}

/// Find the solution for Advent of Code 2017
pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let mut commands: HashMap<i64, (String, String, Option<Value>)> = HashMap::new();
    let mut register_map: HashMap<String, i64> = HashMap::new();
    for (idx, line_result) in reader.lines().enumerate() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        commands.insert(idx as i64, parse_command(line)?);
    }

    initialize_register_map(&commands, &mut register_map)?;

    let mul_count = if second_star {
        let b = 106_700;
        let c = 123_700;
        let mut h = 0;
        for b in (b..=c).step_by(17) {
            if !primal::is_prime(b) {
                h += 1
            }
        }
        h
    } else {
        let mut id = 0;
        let mut count = 0;
        loop {
            if id < 0 || id == commands.len() as i64 {
                break;
            }
            let next_command = commands.get(&id).ok_or(anyhow!("invalid command"))?;
            let (new_id, new_mul_count) = run_command((id, next_command), &mut register_map, count)?;
            id = new_id;
            count = new_mul_count;
        }
        count
    };

    Ok(mul_count)
}

/// Parse a command into (command, register, value)
fn parse_command(command: &str) -> Result<(String, String, Option<Value>)> {
    let token_strs: Vec<&str> = command.split(' ').collect();

    if token_strs.len() == 3 {
        let value = if let Ok(number) = token_strs[2].parse::<i64>() {
            Value::Number(number)
        } else {
            Value::Register(token_strs[2].to_string())
        };
        Ok((token_strs[0].to_string(), token_strs[1].to_string(), Some(value)))
    } else if token_strs.len() == 2 {
        Ok((token_strs[0].to_string(), token_strs[1].to_string(), None))
    } else {
        Err(anyhow!("Invalid command"))
    }
}

/// Initialize the register map.
fn initialize_register_map(commands: &HashMap<i64, (String, String, Option<Value>)>, register_map: &mut HashMap<String, i64>) -> Result<()> {
    for (_, command) in commands.iter() {
        register_map.entry(command.1.clone()).or_insert(0);
    }
    Ok(())
}

/// Run a command
fn run_command((id, command): (i64, &(String, String, Option<Value>)), register_map: &mut HashMap<String, i64>, count: u32) -> Result<(i64, u32)> {
    let cmd = &command.0;
    let register = &command.1;
    let value = &command.2;
    let mut mul_count = count;

    match &cmd[..] {
        "set" => {
            let actual_value = match *value {
                Some(Value::Number(x)) => x,
                Some(Value::Register(ref x)) => *register_map.get(x).ok_or(anyhow!("invalid register"))?,
                _ => return Err(anyhow!("Invalid set command")),
            };
            *register_map.get_mut(register).ok_or(anyhow!("invalid register"))? = actual_value;
        }
        "sub" => {
            let actual_value = match *value {
                Some(Value::Number(x)) => x,
                Some(Value::Register(ref x)) => *register_map.get(x).ok_or(anyhow!("invalid register"))?,
                _ => return Err(anyhow!("Invalid set command")),
            };
            let x = register_map.get_mut(register).ok_or(anyhow!("invalid register"))?;
            *x -= actual_value;
        }
        "mul" => {
            let actual_value = match *value {
                Some(Value::Number(x)) => x,
                Some(Value::Register(ref x)) => *register_map.get(x).ok_or(anyhow!("invalid register"))?,
                _ => return Err(anyhow!("Invalid set command")),
            };
            let x = register_map.get_mut(register).ok_or(anyhow!("invalid register"))?;
            *x *= actual_value;
            mul_count += 1;
        }
        "jnz" => {
            let should_jump = if let Ok(val) = register.parse::<i64>() {
                val
            } else {
                *register_map.get(register).ok_or(anyhow!("invalid register"))?
            };

            if should_jump != 0 {
                let actual_value = match *value {
                    Some(Value::Number(x)) => x,
                    Some(Value::Register(ref x)) => *register_map.get(x).ok_or(anyhow!("invalid register"))?,
                    _ => return Err(anyhow!("Invalid set command")),
                };
                return Ok((id + actual_value, mul_count));
            }
        }
        _ => {}
    }

    Ok((id + 1, mul_count))
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
