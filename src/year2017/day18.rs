//! Advent of Code - Day 18 'Duet' Solution
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

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
    if second_star {
        thread_me()
    } else {
        let mut commands: HashMap<i64, (String, String, Option<Value>)> = HashMap::new();
        let mut register_map: HashMap<String, i64> = HashMap::new();
        for (idx, line_result) in reader.lines().enumerate() {
            let line = &line_result.unwrap_or_else(|_| "".to_string());
            commands.insert(idx as i64, parse_command(line)?);
        }

        initialize_register_map(&commands, &mut register_map)?;

        let mut id = 0;
        loop {
            if id == -1 {
                break;
            }
            let next_command = commands.get(&id).ok_or(anyhow!("invalid command"))?;
            id = run_command((id, next_command), &mut register_map)?;
        }

        let rcv = register_map.get("receive").ok_or(anyhow!("invalid rcv"))?;
        Ok(TryFrom::try_from(*rcv)?)
    }
}

/// Run the threaded version
fn thread_me() -> Result<u32> {
    use std::io::{self, Write};
    let (sender0, receiver0) = channel();
    let (sender1, receiver1) = channel();
    let (sender2, receiver2) = channel();

    thread::spawn(move || {
        let mut commands: HashMap<i64, (String, String, Option<Value>)> = HashMap::new();
        let mut register_map: HashMap<String, i64> = HashMap::new();
        initialize(&mut commands, &mut register_map).expect("");
        *register_map.entry("p".to_string()).or_insert(0) = 0;
        run_solution_in_thread(0, &commands, &mut register_map, &sender0, &receiver1).expect("");
    });
    thread::spawn(move || {
        let mut commands: HashMap<i64, (String, String, Option<Value>)> = HashMap::new();
        let mut register_map: HashMap<String, i64> = HashMap::new();
        initialize(&mut commands, &mut register_map).expect("");
        *register_map.entry("p".to_string()).or_insert(1) = 1;
        if run_solution_in_thread(1, &commands, &mut register_map, &sender1, &receiver0).is_ok() {
            let count = *register_map.get("prog1").ok_or(anyhow!("invalid key")).expect("");
            sender2.send(count).expect("");
        } else {
            sender2.send(-1).expect("");
        }
    });

    if receiver2.recv_timeout(Duration::from_millis(5_000)).is_ok() {
        Ok(0)
    } else {
        writeln!(io::stdout())?;
        Ok(1)
    }
}

/// Initialize
fn initialize(commands: &mut HashMap<i64, (String, String, Option<Value>)>, register_map: &mut HashMap<String, i64>) -> Result<()> {
    let reader = BufReader::new(File::open("data/2017/day18/data_file")?);

    for (idx, line_result) in reader.lines().enumerate() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        commands.insert(idx as i64, parse_command(line)?);
    }
    initialize_register_map(commands, register_map)?;
    Ok(())
}

/// Run the solution in a thread.
fn run_solution_in_thread(
    prog_id: u8,
    commands: &HashMap<i64, (String, String, Option<Value>)>,
    register_map: &mut HashMap<String, i64>,
    sender: &Sender<i64>,
    receiver: &Receiver<i64>,
) -> Result<()> {
    let mut id = 0;
    loop {
        if id == -1 || id < 0 || id == commands.len() as i64 {
            break;
        }
        let next_command = commands.get(&id).ok_or(anyhow!("invalid command"))?;
        id = run_command_snd_rcv(prog_id, (id, next_command), register_map, sender, receiver)?;
    }
    Ok(())
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
fn run_command((id, command): (i64, &(String, String, Option<Value>)), register_map: &mut HashMap<String, i64>) -> Result<i64> {
    let cmd = &command.0;
    let register = &command.1;
    let value = &command.2;
    let mut last_sound: Option<i64> = None;
    let mut receive: Option<i64> = None;

    match &cmd[..] {
        "set" => {
            let actual_value = match *value {
                Some(Value::Number(x)) => x,
                Some(Value::Register(ref x)) => *register_map.get(x).ok_or(anyhow!("invalid register"))?,
                _ => return Err(anyhow!("Invalid set command")),
            };
            *register_map.get_mut(register).ok_or(anyhow!("invalid register"))? = actual_value;
        }
        "add" => {
            let actual_value = match *value {
                Some(Value::Number(x)) => x,
                Some(Value::Register(ref x)) => *register_map.get(x).ok_or(anyhow!("invalid register"))?,
                _ => return Err(anyhow!("Invalid set command")),
            };
            let x = register_map.get_mut(register).ok_or(anyhow!("invalid register"))?;
            *x += actual_value;
        }
        "mul" => {
            let actual_value = match *value {
                Some(Value::Number(x)) => x,
                Some(Value::Register(ref x)) => *register_map.get(x).ok_or(anyhow!("invalid register"))?,
                _ => return Err(anyhow!("Invalid set command")),
            };
            let x = register_map.get_mut(register).ok_or(anyhow!("invalid register"))?;
            *x *= actual_value;
        }
        "mod" => {
            let actual_value = match *value {
                Some(Value::Number(x)) => x,
                Some(Value::Register(ref x)) => *register_map.get(x).ok_or(anyhow!("invalid register"))?,
                _ => return Err(anyhow!("Invalid set command")),
            };
            let x = register_map.get_mut(register).ok_or(anyhow!("invalid register"))?;
            *x %= actual_value;
        }
        "snd" => {
            let snd = register_map.get(register).ok_or(anyhow!("invalid register"))?;
            last_sound = Some(*snd);
        }
        "rcv" => {
            let rcv = register_map.get(register).ok_or(anyhow!("invalid register"))?;

            if *rcv != 0 {
                let last_sound = register_map.get(&"last_sound".to_string()).ok_or(anyhow!("invalid snd"))?;
                receive = Some(*last_sound)
            }
        }
        "jgz" => {
            let should_jump = if let Ok(val) = register.parse::<i64>() {
                val
            } else {
                *register_map.get(register).ok_or(anyhow!("invalid register"))?
            };

            if should_jump > 0 {
                let actual_value = match *value {
                    Some(Value::Number(x)) => x,
                    Some(Value::Register(ref x)) => *register_map.get(x).ok_or(anyhow!("invalid register"))?,
                    _ => return Err(anyhow!("Invalid set command")),
                };
                return Ok(id + actual_value);
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

/// Run a command
fn run_command_snd_rcv(
    prog_id: u8,
    (id, command): (i64, &(String, String, Option<Value>)),
    register_map: &mut HashMap<String, i64>,
    sender: &Sender<i64>,
    receiver: &Receiver<i64>,
) -> Result<i64> {
    use std::io::{self, Write};
    let cmd = &command.0;
    let register = &command.1;
    let value = &command.2;

    match &cmd[..] {
        "set" => {
            let actual_value = match *value {
                Some(Value::Number(x)) => x,
                Some(Value::Register(ref x)) => *register_map.get(x).ok_or(anyhow!("invalid register"))?,
                _ => return Err(anyhow!("Invalid set command")),
            };
            *register_map.get_mut(register).ok_or(anyhow!("invalid register"))? = actual_value;
        }
        "add" => {
            let actual_value = match *value {
                Some(Value::Number(x)) => x,
                Some(Value::Register(ref x)) => *register_map.get(x).ok_or(anyhow!("invalid register"))?,
                _ => return Err(anyhow!("Invalid set command")),
            };
            let x = register_map.get_mut(register).ok_or(anyhow!("invalid register"))?;
            *x += actual_value;
        }
        "mul" => {
            let actual_value = match *value {
                Some(Value::Number(x)) => x,
                Some(Value::Register(ref x)) => *register_map.get(x).ok_or(anyhow!("invalid register"))?,
                _ => return Err(anyhow!("Invalid set command")),
            };
            let x = register_map.get_mut(register).ok_or(anyhow!("invalid register"))?;
            *x *= actual_value;
        }
        "mod" => {
            let actual_value = match *value {
                Some(Value::Number(x)) => x,
                Some(Value::Register(ref x)) => *register_map.get(x).ok_or(anyhow!("invalid register"))?,
                _ => return Err(anyhow!("Invalid set command")),
            };
            let x = register_map.get_mut(register).ok_or(anyhow!("invalid register"))?;
            *x %= actual_value;
        }
        "snd" => {
            if prog_id == 1 {
                let counter = register_map.entry("prog1".to_string()).or_insert(0);
                *counter += 1;
                write!(io::stdout(), "\rCount: {}", *counter)?;
            }
            let snd = register_map.get(register).ok_or(anyhow!("invalid register"))?;
            sender.send(*snd)?;
        }
        "rcv" => {
            let val = receiver.recv()?;
            *register_map.get_mut(register).ok_or(anyhow!("invalid register"))? = val;
        }
        "jgz" => {
            let should_jump = if let Ok(val) = register.parse::<i64>() {
                val
            } else {
                *register_map.get(register).ok_or(anyhow!("invalid register"))?
            };

            if should_jump > 0 {
                let actual_value = match *value {
                    Some(Value::Number(x)) => x,
                    Some(Value::Register(ref x)) => *register_map.get(x).ok_or(anyhow!("invalid register"))?,
                    _ => return Err(anyhow!("Invalid set command")),
                };
                return Ok(id + actual_value);
            }
        }
        _ => {}
    }

    Ok(id + 1)
}

#[cfg(test)]
mod one_star {
    use std::collections::HashMap;

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn solution() {
        let mut commands = HashMap::new();
        let mut register_map: HashMap<String, i64> = HashMap::new();
        let command = super::parse_command("set a 1").expect("");
        assert_eq!(command.0, "set".to_string());
        assert_eq!(command.1, "a");
        assert_eq!(command.2, Some(super::Value::Number(1)));
        commands.insert(0, command);
        let command_1 = super::parse_command("mul a a").expect("");
        assert_eq!(command_1.0, "mul".to_string());
        assert_eq!(command_1.1, "a");
        assert_eq!(command_1.2, Some(super::Value::Register("a".to_string())));
        commands.insert(1, command_1);
        let command_2 = super::parse_command("snd a").expect("");
        assert_eq!(command_2.0, "snd".to_string());
        assert_eq!(command_2.1, "a");
        assert_eq!(command_2.2, None);
        commands.insert(2, command_2);
        super::initialize_register_map(&commands, &mut register_map).expect("");
        assert_eq!(register_map.keys().count(), 1);

        let command_3 = super::parse_command("set a 1").expect("");
        let mut next_id = super::run_command((0, &command_3), &mut register_map).expect("");
        assert_eq!(*register_map.get("a").ok_or(0).expect(""), 1);
        assert_eq!(next_id, 1);
        let command_4 = super::parse_command("add a 2").expect("");
        next_id = super::run_command((next_id, &command_4), &mut register_map).expect("");
        assert_eq!(*register_map.get("a").ok_or(0).expect(""), 3);
        assert_eq!(next_id, 2);
        let command_5 = super::parse_command("mul a a").expect("");
        next_id = super::run_command((next_id, &command_5), &mut register_map).expect("");
        assert_eq!(*register_map.get("a").ok_or(0).expect(""), 9);
        assert_eq!(next_id, 3);
        let command_6 = super::parse_command("mod a 5").expect("");
        next_id = super::run_command((next_id, &command_6), &mut register_map).expect("");
        assert_eq!(*register_map.get("a").ok_or(0).expect(""), 4);
        assert_eq!(next_id, 4);
        let command_7 = super::parse_command("snd a").expect("");
        next_id = super::run_command((next_id, &command_7), &mut register_map).expect("");
        assert_eq!(*register_map.get("a").ok_or(0).expect(""), 4);
        assert_eq!(*register_map.get("last_sound").ok_or(0).expect(""), 4);
        assert_eq!(next_id, 5);
        let command_8 = super::parse_command("set a 0").expect("");
        next_id = super::run_command((next_id, &command_8), &mut register_map).expect("");
        assert_eq!(*register_map.get("a").ok_or(1).expect(""), 0);
        assert_eq!(next_id, 6);
        let command_9 = super::parse_command("rcv a").expect("");
        next_id = super::run_command((next_id, &command_9), &mut register_map).expect("");
        assert_eq!(*register_map.get("a").ok_or(1).expect(""), 0);
        assert_eq!(next_id, 7);
        let command_10 = super::parse_command("jgz a -1").expect("");
        next_id = super::run_command((next_id, &command_10), &mut register_map).expect("");
        assert_eq!(*register_map.get("a").ok_or(1).expect(""), 0);
        assert_eq!(next_id, 8);
        let command_11 = super::parse_command("set a 1").expect("");
        next_id = super::run_command((next_id, &command_11), &mut register_map).expect("");
        assert_eq!(*register_map.get("a").ok_or(0).expect(""), 1);
        assert_eq!(next_id, 9);
        let command_12 = super::parse_command("jgz a -2").expect("");
        next_id = super::run_command((next_id, &command_12), &mut register_map).expect("");
        assert_eq!(*register_map.get("a").ok_or(0).expect(""), 1);
        assert_eq!(next_id, 7);
        let command_13 = super::parse_command("jgz a -1").expect("");
        next_id = super::run_command((next_id, &command_13), &mut register_map).expect("");
        assert_eq!(*register_map.get("a").ok_or(0).expect(""), 1);
        assert_eq!(next_id, 6);
        let command_14 = super::parse_command("rcv a").expect("");
        next_id = super::run_command((next_id, &command_14), &mut register_map).expect("");
        assert_eq!(*register_map.get("a").ok_or(0).expect(""), 1);
        assert_eq!(next_id, -1);
        assert_eq!(*register_map.get("receive").ok_or(0).expect(""), 4);
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {}
}
