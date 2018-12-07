//! Advent of Code - Day 7 "The Sum of Its Parts" Solution
use error::Result;
use indexmap::IndexSet;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::io::BufRead;

pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    if second_star {
        let duration = find_duration(reader, false)?;
        Ok(duration)
    } else {
        let order = find_order(reader)?;
        println!("{}", order);
        Ok(0)
    }
}

fn duration_of(val: &str, base: u32) -> Result<u32> {
    Ok(match val {
        "A" => base + 1,
        "B" => base + 2,
        "C" => base + 3,
        "D" => base + 4,
        "E" => base + 5,
        "F" => base + 6,
        "G" => base + 7,
        "H" => base + 8,
        "I" => base + 9,
        "J" => base + 10,
        "K" => base + 11,
        "L" => base + 12,
        "M" => base + 13,
        "N" => base + 14,
        "O" => base + 15,
        "P" => base + 16,
        "Q" => base + 17,
        "R" => base + 18,
        "S" => base + 19,
        "T" => base + 20,
        "U" => base + 21,
        "V" => base + 22,
        "W" => base + 23,
        "X" => base + 24,
        "Y" => base + 25,
        "Z" => base + 26,
        _ => return Err("invalid instructions".into()),
    })
}

#[derive(Clone, Debug, Default, Getters, Setters)]
struct Worker {
    id: u32,
    #[get]
    #[set]
    work: Option<(String, u32)>,
    #[get]
    remaining: u32,
}

impl fmt::Display for Worker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref work) = self.work {
            write!(f, "{}: ({},{}) -> {}", self.id, work.0, work.1, self.remaining)
        } else {
            write!(f, "{}: None -> {}", self.id, self.remaining)
        }
    }
}

fn find_duration<T: BufRead>(reader: T, test: bool) -> Result<u32> {
    let line_re = Regex::new(r#"Step ([A-Z]) must be finished before step ([A-Z])"#)?;
    let base = if test { 0 } else { 60 };
    let workers_count = if test { 2 } else { 5 };
    let mut workers = Vec::new();

    for i in 0..workers_count {
        let mut worker = Worker::default();
        worker.id = i;
        workers.push(worker);
    }

    let mut child_map = HashMap::new();
    let mut parents_map = HashMap::new();
    let mut pending = IndexSet::new();

    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            for cap in line_re.captures_iter(&line) {
                let first = (&cap[1]).to_string();
                let first_duration = duration_of(&first, base)?;
                let first_tuple = (first, first_duration);
                let second = (&cap[2]).to_string();
                let second_duration = duration_of(&second, base)?;
                let second_tuple = (second, second_duration);
                pending.insert(first_tuple.clone());
                pending.insert(second_tuple.clone());
                let mut children = child_map.entry(first_tuple.clone()).or_insert_with(|| vec![]);
                children.push(second_tuple.clone());
                let mut parents = parents_map.entry(second_tuple).or_insert_with(|| vec![]);
                parents.push(first_tuple);
            }
        } else {
            return Err("unable to parse input".into());
        }
    }

    let all_children: IndexSet<(String, u32)> = child_map.iter().flat_map(|(_, c)| c).cloned().collect();
    let mut ready: IndexSet<(String, u32)> = pending.difference(&all_children).cloned().collect();
    let mut complete: IndexSet<(String, u32)> = IndexSet::new();

    ready.sort_by(|x, y| x.cmp(y).reverse());

    let mut total_ticks = 0;
    for tick in 0.. {
        // Check for completed work and adjust the ready and complete queues as necessary
        let complete_work = complete_work(&mut workers);

        for work in complete_work {
            if let Some(children) = child_map.get(&work) {
                for child in children {
                    ready.insert(child.clone());
                }

                // Sort the ready work as new children have been added to the queue.
                ready.sort_by(|x, y| x.cmp(y).reverse());
            }

            complete.insert(work);
        }

        // Assign work to idle workers if conditions are favorable.
        'outer: for mut worker in &mut workers {
            if let Some(nx) = ready.pop() {
                // Check that all the parent steps have completed.  If not, move on.
                if let Some(parents) = parents_map.get(&nx) {
                    for parent in parents {
                        if !complete.contains(parent) {
                            continue 'outer;
                        }
                    }
                }

                // Cosume ready work if the worker is idle.  Otherwise, push the ready
                // back onto the queue, sort,  and check the next worker.
                if !consume_work(worker, &nx) {
                    ready.insert(nx);
                    ready.sort_by(|x, y| x.cmp(y).reverse());
                    continue;
                }
            }
        }

        // Complete 1 second of work on each busy worker
        adjust_workers(&mut workers);

        // println!();
        // println!("Tick: {}", tick);
        // println!("Ready: {:?}", ready);
        // println!("Complete: {:?}", complete);

        // for worker in &workers {
        //     println!("{}", worker);
        // }

        // Are we done?
        if ready.is_empty() && all_idle(&workers) {
            total_ticks = tick;
            break;
        }
    }

    Ok(total_ticks)
}

fn all_idle(workers: &[Worker]) -> bool {
    workers.iter().all(|worker| worker.work().is_none())
}

fn adjust_workers(workers: &mut Vec<Worker>) {
    for worker in workers {
        if worker.work.is_some() {
            worker.remaining -= 1;
        }
    }
}

fn complete_work(workers: &mut Vec<Worker>) -> Vec<(String, u32)> {
    let mut result = Vec::new();

    for worker in workers {
        let mut clear = false;
        if let Some(work) = worker.work() {
            if worker.remaining == 0 {
                result.push(work.clone());
                clear = true;
            }
        }

        if clear {
            worker.set_work(None);
        }
    }

    result
}

fn consume_work(worker: &mut Worker, work: &(String, u32)) -> bool {
    if worker.work.is_none() {
        worker.remaining = work.1;
        worker.work = Some(work.clone());
        true
    } else {
        false
    }
}

fn find_order<T: BufRead>(reader: T) -> Result<String> {
    let line_re = Regex::new(r#"Step ([A-Z]) must be finished before step ([A-Z])"#)?;
    let mut child_map = HashMap::new();
    let mut parents_map = HashMap::new();
    let mut node_set = IndexSet::new();

    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            for cap in line_re.captures_iter(&line) {
                let first = (&cap[1]).to_string();
                let second = (&cap[2]).to_string();
                node_set.insert(first.clone());
                node_set.insert(second.clone());
                let mut children = child_map.entry(first.clone()).or_insert_with(|| vec![]);
                children.push(second.clone());
                let mut parents = parents_map.entry(second).or_insert_with(|| vec![]);
                parents.push(first);
            }
        } else {
            return Err("unable to parse input".into());
        }
    }

    let all_children: IndexSet<String> = child_map.iter().flat_map(|(_, c)| c).cloned().collect();
    let mut ready: IndexSet<String> = node_set.difference(&all_children).cloned().collect();
    let mut instruction_order = String::new();
    ready.sort_by(|x, y| x.cmp(y).reverse());

    'outer: while let Some(nx) = ready.pop() {
        // Check that all the parent steps have completed.  If not, move to next
        // ready step.
        if let Some(parents) = parents_map.get(&nx) {
            for parent in parents {
                if !instruction_order.contains(parent) {
                    continue 'outer;
                }
            }
        }

        // All the parents have completed, so complete this step and push
        // the children into the ready state.  Sort the readies.
        instruction_order.push_str(&nx);

        if let Some(children) = child_map.get(&nx) {
            for child in children {
                ready.insert(child.clone());
            }
        }

        ready.sort_by(|x, y| x.cmp(y).reverse());
    }

    Ok(instruction_order)
}

#[cfg(test)]
mod one_star {
    use super::find_order;
    use error::Result;
    use std::io::Cursor;

    const TEST_CHAIN: &str = r"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_order(Cursor::new(TEST_CHAIN))?, "CABDFE".to_string());
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find_duration;
    use error::Result;
    use std::io::Cursor;

    const TEST_CHAIN: &str = r"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_duration(Cursor::new(TEST_CHAIN), true)?, 15);
        Ok(())
    }
}
