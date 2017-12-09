//! Advent of Code - Day 8 Solution
use error::{Error, Result};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::BufRead;

/// Register commands
enum Command {
    /// Increment
    Inc,
    /// Decrement
    Dec,
}

impl<'a> TryFrom<&'a str> for Command {
    type Error = Error;
    fn try_from(command: &str) -> Result<Self> {
        match command {
            "inc" => Ok(Command::Inc),
            "dec" => Ok(Command::Dec),
            _ => Err("Invalid command!".into()),
        }
    }
}

/// Supported condition operators
enum Operator {
    /// `>`
    GreaterThan,
    /// `>=`
    GreaterThanEqualTo,
    /// `==`
    EqualTo,
    /// `<=`
    LessThanEqualTo,
    /// `<`
    LessThan,
    /// `!=`
    NotEqualTo,
}

impl<'a> TryFrom<&'a str> for Operator {
    type Error = Error;
    fn try_from(command: &str) -> Result<Self> {
        match command {
            ">" => Ok(Operator::GreaterThan),
            ">=" => Ok(Operator::GreaterThanEqualTo),
            "==" => Ok(Operator::EqualTo),
            "<=" => Ok(Operator::LessThanEqualTo),
            "<" => Ok(Operator::LessThan),
            "!=" => Ok(Operator::NotEqualTo),
            _ => Err("Invalid operator!".into()),
        }
    }
}

/// A condition that must be met before operating on a register.
struct Condition {
    /// The register to check.
    register: String,
    /// The operater, e.g. `<`, `>`
    op: Operator,
    /// The value on the right of the condition.
    value: u32,
}

/// A complete register command.
struct RegisterCommand {
    /// The command (inc or dec)
    command: Command,
    /// The value to use with the command.
    value: i32,
    /// The condition that has to be met before the command is applied.
    condition: Condition,
}

/// Calculate the largest value in a register.
pub fn largest_register_value<T: BufRead>(reader: T, _second_star: bool) -> Result<u32> {
    let mut register_map = HashMap::new();
    let mut commands = Vec::new();

    for line_result in reader.lines() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        generate_register_map_entry(line, &mut register_map, &mut commands)?;
    }
    Ok(0)
}

/// Generate a register map entry
fn generate_register_map_entry(
    line: &str,
    register_map: &mut HashMap<String, u32>,
    commands: &mut Vec<RegisterCommand>,
) -> Result<()> {
    let line_desc: Vec<&str> = line.split_whitespace().collect();
    let name_str = line_desc.get(0).ok_or("Invalid register name!")?;
    let name = String::from(*name_str);
    let command_str = line_desc.get(1).ok_or("Invalid command!")?;
    let value = line_desc
        .get(2)
        .ok_or("Invalid command value")?
        .parse::<i32>()?;
    let command_register_str = line_desc.get(4).ok_or("Invalid command register!")?;
    let command_register = String::from(*command_register_str);
    let operator_str = line_desc.get(5).ok_or("Invalid operator!")?;

    register_map.entry(name).or_insert(0);

    commands.push(RegisterCommand {
        command: TryFrom::try_from(*command_str)?,
        value: value,
        condition: Condition {
            register: command_register,
            op: TryFrom::try_from(*operator_str)?,
            value: 1,
        },
    });

    Ok(())
}

#[cfg(test)]
mod test {
    use super::{generate_register_map_entry, RegisterCommand};
    use error::Result;
    use std::collections::HashMap;

    fn generate_map_entry(
        register_map: &mut HashMap<String, u32>,
        commands: &mut Vec<RegisterCommand>,
    ) -> Result<()> {
        generate_register_map_entry("b inc 5 if a > 1", register_map, commands)?;
        generate_register_map_entry("a inc 1 if b < 5", register_map, commands)?;
        generate_register_map_entry("c dec -10 if a >= 1", register_map, commands)?;
        generate_register_map_entry("c inc -20 if c == 1", register_map, commands)?;
        Ok(())
    }

    #[test]
    fn register_list() {
        let mut register_map = HashMap::new();
        let mut commands = Vec::new();
        generate_map_entry(&mut register_map, &mut commands).expect("");
        assert_eq!(register_map.len(), 3);
        assert_eq!(commands.len(), 4);
        assert_eq!(*register_map.get(&"a".to_string()).ok_or("").expect(""), 0);
        assert_eq!(*register_map.get(&"b".to_string()).ok_or("").expect(""), 0);
        assert_eq!(*register_map.get(&"c".to_string()).ok_or("").expect(""), 0);
    }
}
