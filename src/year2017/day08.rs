//! Advent of Code - Day 8 "I Heard You Like Registers" Solution
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
    value: i32,
}

/// A complete register command.
struct RegisterCommand {
    /// The name of the register to operate on.
    register: String,
    /// The command (inc or dec)
    command: Command,
    /// The value to use with the command.
    value: i32,
    /// The condition that has to be met before the command is applied.
    condition: Condition,
}

/// Calculate the largest value in a register.
pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let mut register_map = HashMap::new();
    let mut commands = Vec::new();

    for line_result in reader.lines() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        generate_register_map_entry_and_command(line, &mut register_map, &mut commands)?;
    }

    if second_star {
        let mut maximum_attained = i32::min_value();

        for command in &commands {
            if check_condition(&register_map, &command.condition).expect("") {
                execute_command(&mut register_map, command).expect("");
            }
            let max = register_map.values().max().ok_or("No max found").expect("");

            if *max > maximum_attained {
                maximum_attained = *max;
            }
        }

        Ok(TryFrom::try_from(maximum_attained)?)
    } else {
        for command in &commands {
            if check_condition(&register_map, &command.condition)? {
                execute_command(&mut register_map, command)?
            }
        }

        let max = register_map.values().max().ok_or("No max found")?;
        Ok(TryFrom::try_from(*max)?)
    }
}

/// Generate a register map entry and the associated command
fn generate_register_map_entry_and_command(line: &str, register_map: &mut HashMap<String, i32>, commands: &mut Vec<RegisterCommand>) -> Result<()> {
    let line_desc: Vec<&str> = line.split_whitespace().collect();
    let name_str = line_desc.get(0).ok_or("Invalid register name!")?;
    let name = String::from(*name_str);
    let command_str = line_desc.get(1).ok_or("Invalid command!")?;
    let value = line_desc.get(2).ok_or("Invalid command value")?.parse::<i32>()?;
    let command_register_str = line_desc.get(4).ok_or("Invalid command register!")?;
    let command_register = String::from(*command_register_str);
    let operator_str = line_desc.get(5).ok_or("Invalid operator!")?;
    let condition_value = line_desc.get(6).ok_or("Invalid condition value!")?.parse::<i32>()?;

    register_map.entry(name.clone()).or_insert(0);

    commands.push(RegisterCommand {
        register: name,
        command: TryFrom::try_from(*command_str)?,
        value,
        condition: Condition {
            register: command_register,
            op: TryFrom::try_from(*operator_str)?,
            value: condition_value,
        },
    });

    Ok(())
}

/// Check the command condition
fn check_condition(register_map: &HashMap<String, i32>, condition: &Condition) -> Result<bool> {
    let register_value = register_map.get(&condition.register).ok_or("Cannot read value from register")?;
    let condition_value = &condition.value;

    match condition.op {
        Operator::GreaterThan => Ok(register_value > condition_value),
        Operator::GreaterThanEqualTo => Ok(register_value >= condition_value),
        Operator::EqualTo => Ok(register_value == condition_value),
        Operator::LessThanEqualTo => Ok(register_value <= condition_value),
        Operator::LessThan => Ok(register_value < condition_value),
        Operator::NotEqualTo => Ok(register_value != condition_value),
    }
}

/// Execute the given command
fn execute_command(register_map: &mut HashMap<String, i32>, register_command: &RegisterCommand) -> Result<()> {
    let map_entry = register_map.entry(register_command.register.clone()).or_insert(0);

    match register_command.command {
        Command::Inc => *map_entry += register_command.value,
        Command::Dec => *map_entry -= register_command.value,
    }

    Ok(())
}

#[cfg(test)]
fn generate_map_entry(register_map: &mut HashMap<String, i32>, commands: &mut Vec<RegisterCommand>) -> Result<()> {
    generate_register_map_entry_and_command("b inc 5 if a > 1", register_map, commands)?;
    generate_register_map_entry_and_command("a inc 1 if b < 5", register_map, commands)?;
    generate_register_map_entry_and_command("c dec -10 if a >= 1", register_map, commands)?;
    generate_register_map_entry_and_command("c inc -20 if c == 10", register_map, commands)?;
    Ok(())
}

#[cfg(test)]
mod one_star {
    use std::collections::HashMap;

    #[test]
    fn solution() {
        let mut register_map = HashMap::new();
        let mut commands = Vec::new();
        super::generate_map_entry(&mut register_map, &mut commands).expect("");
        assert_eq!(register_map.len(), 3);
        assert_eq!(commands.len(), 4);
        assert_eq!(*register_map.get(&"a".to_string()).ok_or("").expect(""), 0);
        assert_eq!(*register_map.get(&"b".to_string()).ok_or("").expect(""), 0);
        assert_eq!(*register_map.get(&"c".to_string()).ok_or("").expect(""), 0);

        for command in &commands {
            if super::check_condition(&register_map, &command.condition).expect("") {
                super::execute_command(&mut register_map, command).expect("");
            }
        }

        let max = register_map.values().max().ok_or("No max found").expect("");
        assert_eq!(*max, 1);
    }
}

#[cfg(test)]
mod two_star {
    use std::collections::HashMap;

    #[test]
    fn solution() {
        let mut register_map = HashMap::new();
        let mut commands = Vec::new();
        super::generate_map_entry(&mut register_map, &mut commands).expect("");
        assert_eq!(register_map.len(), 3);
        assert_eq!(commands.len(), 4);
        assert_eq!(*register_map.get(&"a".to_string()).ok_or("").expect(""), 0);
        assert_eq!(*register_map.get(&"b".to_string()).ok_or("").expect(""), 0);
        assert_eq!(*register_map.get(&"c".to_string()).ok_or("").expect(""), 0);

        let mut maximum_attained = ::std::i32::MIN;

        for command in &commands {
            if super::check_condition(&register_map, &command.condition).expect("") {
                super::execute_command(&mut register_map, command).expect("");
            }
            let max = register_map.values().max().ok_or("No max found").expect("");

            if *max > maximum_attained {
                maximum_attained = *max;
            }
        }

        let max = register_map.values().max().ok_or("No max found").expect("");
        assert_eq!(*max, 1);
        assert_eq!(maximum_attained, 10);
    }
}
