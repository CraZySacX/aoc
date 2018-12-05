use chrono::{TimeZone, Timelike, Utc};
use error::Result;
use regex::Regex;
use std::collections::BTreeMap;
use std::fmt;
use std::io::BufRead;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
struct Guard {
    id: u32,
    minute_map: BTreeMap<u32, u32>
}

impl fmt::Display for Guard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Guard #{}", self.id)
    }
}

pub fn find_solution<T: BufRead>(reader: T, _second_star: bool) -> Result<u32> {
    let line_re = Regex::new(r#"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (.*)"#)?;
    let guard_re = Regex::new(r#"Guard #(\d+) begins shift"#)?;
    let falls_re = Regex::new(r#"falls asleep"#)?;
    let wakes_re = Regex::new(r#"wakes up"#)?;

    let mut sorted_events = BTreeMap::new();

    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            for cap in line_re.captures_iter(&line) {
                let y = (&cap[1]).parse::<i32>()?;
                let mon = (&cap[2]).parse::<u32>()?;
                let d = (&cap[3]).parse::<u32>()?;
                let h = (&cap[4]).parse::<u32>()?;
                let m = (&cap[5]).parse::<u32>()?;
                let rest = &cap[6];

                let dt = Utc.ymd(y, mon, d).and_hms(h, m, 0);
                sorted_events.insert(dt, rest.to_string());
            }
        }
    }

    for (dt, evt) in &sorted_events {
        println!("{}: {}", dt, evt);
    }

    let mut guards_napping = Vec::new();
    let mut current_guard = Guard::default();
    let mut minute_asleep = 0;
    for (dt, evt) in sorted_events.iter().filter(|(dt, _)| (**dt).hour() == 0) {
        if guard_re.is_match(evt) {
            for cap in guard_re.captures_iter(evt) {
                current_guard.id = (&cap[1]).parse::<u32>()?;
                println!("{} is on duty", current_guard);
                guards_napping.push(current_guard);
            }
        } else if current_guard.id > 0 {
            if falls_re.is_match(evt) {
                println!("{} fell asleep at 00:{}", current_guard, dt.minute());
                minute_asleep = dt.minute();
            } else if wakes_re.is_match(evt) {
                println!("{} wakes up at 00:{}", current_guard, dt.minute());
                let mut guard_on_duty = guards_napping.pop();
                *guards_napping.entry(current_guard.id).or_insert(0) += dt.minute() - minute_asleep - 1;
            }
        }
    }

    for (id, time_napping) in guards_napping {
        println!("{}: {}", id, time_napping);
    }
    Ok(1)
}
