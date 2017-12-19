//! Advent of Code - Day 18 'Duet' Solution
use error::Result;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::BufRead;

/// A value can either be a pointer to a register or a number.
#[derive(Debug, PartialEq)]
enum Value {
    /// A number value.
    Number(i32),
    /// A registe pointer value.
    Register(String),
}

/// Find the solution for Advent of Code 2017
pub fn find_solution<T: BufRead>(reader: T, _second_star: bool) -> Result<u32> {
    let mut commands: HashMap<i32, (String, String, Option<Value>)> = HashMap::new();
    let mut register_map: HashMap<String, i32> = HashMap::new();
    for (idx, line_result) in reader.lines().enumerate() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        commands.insert(TryFrom::try_from(idx)?, parse_command(line)?);
    }

    // TODO: Commands should be a hashmap of i32 -> command, so jumps can be based on id.
    // A jump to -1 indicates completion.
    initialize_register_map(&commands, &mut register_map)?;

    // TODO: Once commands have id, make this a loop, that runs until a valid 'rcv'.
    for command in commands {
        let next_id = run_command(command, &mut register_map)?;
    }
    Ok(0)
}

/// Parse a command into (command, register, value)
fn parse_command(command: &str) -> Result<(String, String, Option<Value>)> {
    let token_strs: Vec<&str> = command.split(' ').collect();

    if token_strs.len() == 3 {
        let value = if let Ok(number) = token_strs[2].parse::<i32>() {
            Value::Number(number)
        } else {
            Value::Register(token_strs[2].to_string())
        };
        Ok((
            token_strs[0].to_string(),
            token_strs[1].to_string(),
            Some(value),
        ))
    } else if token_strs.len() == 2 {
        Ok((token_strs[0].to_string(), token_strs[1].to_string(), None))
    } else {
        Err("Invalid command".into())
    }
}

/// Initialize the register map.
fn initialize_register_map(commands: &HashMap<i32, (String, String, Option<Value>)>, register_map: &mut HashMap<String, i32>) -> Result<()> {
    for (_, command) in commands.iter() {
        register_map.entry(command.1.clone()).or_insert(0);
    }
    Ok(())
}

/// Run a command
fn run_command<'a>((id, command): (i32, (String, String, Option<Value>)), register_map: &'a mut HashMap<String, i32>) -> Result<i32> {
    let cmd = command.0;
    let register = command.1;
    let value = command.2;
    let mut last_sound: Option<i32> = None;
    let mut receive: Option<i32> = None;

    match &cmd[..] {
        "set" => {
            let actual_value = match value {
                Some(Value::Number(x)) => x,
                Some(Value::Register(x)) => *register_map.get(&x).ok_or("invalid register")?,
                _ => return Err("Invalid set command".into()),
            };
            *register_map.get_mut(&register).ok_or("invalid register")? = actual_value;
        }
        "add" => {
            let actual_value = match value {
                Some(Value::Number(x)) => x,
                Some(Value::Register(x)) => *register_map.get(&x).ok_or("invalid register")?,
                _ => return Err("Invalid set command".into()),
            };
            let x = register_map.get_mut(&register).ok_or("invalid register")?;
            *x = *x + actual_value;
        }
        "mul" => {
            let actual_value = match value {
                Some(Value::Number(x)) => x,
                Some(Value::Register(x)) => *register_map.get(&x).ok_or("invalid register")?,
                _ => return Err("Invalid set command".into()),
            };
            let x = register_map.get_mut(&register).ok_or("invalid register")?;
            *x = *x * actual_value;
        }
        "mod" => {
            let actual_value = match value {
                Some(Value::Number(x)) => x,
                Some(Value::Register(x)) => *register_map.get(&x).ok_or("invalid register")?,
                _ => return Err("Invalid set command".into()),
            };
            let x = register_map.get_mut(&register).ok_or("invalid register")?;
            *x = *x % actual_value;
        }
        "snd" => {
            let snd = register_map.get(&register).ok_or("invalid register")?;
            last_sound = Some(*snd);
        }
        "rcv" => {
            let rcv = register_map.get(&register).ok_or("invalid register")?;

            if *rcv != 0 {
                let last_sound = register_map
                    .get(&"last_sound".to_string())
                    .ok_or("invalid snd")?;
                receive = Some(*last_sound)
            }
        },
        "jgz" => {
            let should_jump = register_map.get(&register).ok_or("invalid register")?;

            if *should_jump > 0 {
                // Jump, by setting next_id appropriately
            }
        }
        _ => {}
    }

    if let Some(sound) = last_sound {
        register_map.insert("last_sound".to_string(), sound);
    }

    if let Some(rcv) = receive {
        register_map.insert("receive".to_string(), rcv);
        Ok(-1)
    } else {
        Ok(id + 1)
    }
}

#[cfg(test)]
mod one_star {
    use std::collections::HashMap;

    #[test]
    fn solution() {
        let mut commands = HashMap::new();
        let mut register_map: HashMap<String, i32> = HashMap::new();
        let command = super::parse_command("set a 1").expect("");
        assert_eq!(command.0, "set".to_string());
        assert_eq!(command.1, "a".to_string());
        assert_eq!(command.2, Some(super::Value::Number(1)));
        commands.insert(0, command);
        let command_1 = super::parse_command("mul a a").expect("");
        assert_eq!(command_1.0, "mul".to_string());
        assert_eq!(command_1.1, "a".to_string());
        assert_eq!(command_1.2, Some(super::Value::Register("a".to_string())));
        commands.insert(1, command_1);
        let command_2 = super::parse_command("snd a").expect("");
        assert_eq!(command_2.0, "snd".to_string());
        assert_eq!(command_2.1, "a".to_string());
        assert_eq!(command_2.2, None);
        commands.insert(2, command_2);
        super::initialize_register_map(&commands, &mut register_map).expect("");
        assert_eq!(register_map.keys().count(), 1);

        let command_3 = super::parse_command("set a 1").expect("");
        let mut next_id = super::run_command((0, command_3), &mut register_map).expect("");
        assert_eq!(*register_map.get(&"a".to_string()).ok_or(0).expect(""), 1);
        assert_eq!(next_id, 1);
        let command_4 = super::parse_command("add a 2").expect("");
        next_id = super::run_command((next_id, command_4), &mut register_map).expect("");
        assert_eq!(*register_map.get(&"a".to_string()).ok_or(0).expect(""), 3);
        assert_eq!(next_id, 2);
        let command_5 = super::parse_command("mul a a").expect("");
        next_id = super::run_command((next_id, command_5), &mut register_map).expect("");
        assert_eq!(*register_map.get(&"a".to_string()).ok_or(0).expect(""), 9);
        assert_eq!(next_id, 3);
        let command_6 = super::parse_command("mod a 5").expect("");
        next_id = super::run_command((next_id, command_6), &mut register_map).expect("");
        assert_eq!(*register_map.get(&"a".to_string()).ok_or(0).expect(""), 4);
        assert_eq!(next_id, 4);
        let command_7 = super::parse_command("snd a").expect("");
        next_id = super::run_command((next_id, command_7), &mut register_map).expect("");
        assert_eq!(*register_map.get(&"a".to_string()).ok_or(0).expect(""), 4);
        assert_eq!(
            *register_map
                .get(&"last_sound".to_string())
                .ok_or(0)
                .expect(""),
            4
        );
        assert_eq!(next_id, 5);
        let command_8 = super::parse_command("set a 0").expect("");
        next_id = super::run_command((next_id, command_8), &mut register_map).expect("");
        assert_eq!(*register_map.get(&"a".to_string()).ok_or(1).expect(""), 0);
        assert_eq!(next_id, 6);
        let command_9 = super::parse_command("rcv a").expect("");
        next_id = super::run_command((next_id, command_9), &mut register_map).expect("");
        assert_eq!(*register_map.get(&"a".to_string()).ok_or(1).expect(""), 0);
        assert_eq!(next_id, 7);
        let command_10 = super::parse_command("jgz a -l").expect("");
        next_id = super::run_command((next_id, command_10), &mut register_map).expect("");
        assert_eq!(*register_map.get(&"a".to_string()).ok_or(1).expect(""), 0);
        assert_eq!(next_id, 8);
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {
        assert!(true);
    }
}
