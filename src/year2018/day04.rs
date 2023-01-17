//! Advent of Code - Day 4 "Repose Record" Solution
use anyhow::{anyhow, Result};
use chrono::{TimeZone, Timelike, Utc};
use regex::Regex;
use std::collections::BTreeMap;
use std::fmt;
use std::io::BufRead;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
struct Guard {
    id: u32,
    minute_map: BTreeMap<u32, u32>,
}

impl fmt::Display for Guard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Guard #{}", self.id)
    }
}

pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let line_re = Regex::new(r#"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (.*)"#)?;
    let guard_re = Regex::new(r#"Guard #(\d+) begins shift"#)?;
    let mut sorted_events = BTreeMap::new();

    for line in reader.lines().flatten() {
        for cap in line_re.captures_iter(&line) {
            let y = (cap[1]).parse::<i32>()?;
            let mon = (cap[2]).parse::<u32>()?;
            let d = (cap[3]).parse::<u32>()?;
            let h = (cap[4]).parse::<u32>()?;
            let m = (cap[5]).parse::<u32>()?;
            let rest = &cap[6];
            let dt = Utc.with_ymd_and_hms(y, mon, d, h, m, 0).single().unwrap_or_default();

            sorted_events.insert(dt, rest.to_string());
        }
    }

    let mut guards_napping: BTreeMap<u32, BTreeMap<u32, u32>> = BTreeMap::new();
    let mut current_guard = 0;
    let mut minute_asleep = 0;
    for (dt, evt) in sorted_events.iter() {
        if guard_re.is_match(evt) {
            for cap in guard_re.captures_iter(evt) {
                current_guard = (cap[1]).parse::<u32>()?;
                guards_napping.entry(current_guard).or_insert_with(|| {
                    let mut minute_map = BTreeMap::new();
                    for i in 0..60 {
                        minute_map.insert(i, 0);
                    }
                    minute_map
                });
            }
        } else if current_guard > 0 && dt.hour() == 0 {
            if evt.contains("falls asleep") {
                minute_asleep = dt.minute();
            } else if evt.contains("wakes up") {
                let minutes_map = guards_napping.get_mut(&current_guard).ok_or(anyhow!("Invalid Index"))?;
                for i in minute_asleep..dt.minute() {
                    *minutes_map.entry(i).or_insert(0) += 1;
                }
            }
        }
    }

    let mut max_id = 0;
    let mut max_minute_asleep = 0;
    if second_star {
        let mut max_times_asleep = 0;
        for (id, time_napping) in guards_napping {
            let (mma, mta): (u32, u32) = time_napping.iter().max_by_key(|(_, v)| *v).map_or((0, 0), |(x, y)| (*x, *y));

            if mta > max_times_asleep {
                max_times_asleep = mta;
                max_minute_asleep = mma;
                max_id = id;
            }
        }
    } else {
        let mut max_time_asleep = 0;
        for (id, time_napping) in guards_napping {
            let total_time_asleep: u32 = time_napping.values().sum();
            let (mma, _): (u32, u32) = time_napping.iter().max_by_key(|(_, v)| *v).map_or((0, 0), |(x, y)| (*x, *y));
            if total_time_asleep > max_time_asleep {
                max_id = id;
                max_time_asleep = total_time_asleep;
                max_minute_asleep = mma;
            }
        }
    }

    Ok(max_id * max_minute_asleep)
}

#[cfg(test)]
const TEST_LINES: &str = r#"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up"#;

#[cfg(test)]
mod one_star {
    use super::{find_solution, TEST_LINES};
    use anyhow::Result;
    use std::io::Cursor;

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_solution(Cursor::new(TEST_LINES), false)?, 240);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find_solution, TEST_LINES};
    use anyhow::Result;
    use std::io::Cursor;

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_solution(Cursor::new(TEST_LINES), true)?, 4455);
        Ok(())
    }
}
