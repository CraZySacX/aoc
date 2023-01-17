//! Advent of Code - Day 15 "Beverage Bandits" Solution
use anyhow::{anyhow, Result};
use ndarray::{Array2, Axis, Zip};
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::io::BufRead;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum UnitKind {
    Elf,
    Goblin,
}

impl fmt::Display for UnitKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ch = match self {
            UnitKind::Elf => 'E',
            UnitKind::Goblin => 'G',
        };
        write!(f, "{ch}")
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Unit {
    kind: UnitKind,
    attack_power: usize,
    hit_points: usize,
    has_moved: bool,
    has_attacked: bool,
}

impl Unit {
    fn new_elf(attack_power: usize) -> Self {
        Self {
            kind: UnitKind::Elf,
            attack_power,
            hit_points: 200,
            has_moved: false,
            has_attacked: false,
        }
    }

    fn new_goblin() -> Self {
        Self {
            kind: UnitKind::Goblin,
            attack_power: 3,
            hit_points: 200,
            has_moved: false,
            has_attacked: false,
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum ElementKind {
    #[default]
    Cavern,
    Wall,
    Unit,
}

impl fmt::Display for ElementKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ch = match self {
            ElementKind::Cavern => '.',
            ElementKind::Wall => '#',
            ElementKind::Unit => 'U',
        };
        write!(f, "{ch}")
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Element {
    kind: ElementKind,
    unit: Option<Unit>,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref u) = self.unit {
            write!(f, "{u}")
        } else {
            write!(f, "{}", self.kind)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Action {
    Attack([usize; 2]),
    Move([usize; 2]),
    No,
}

pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let outcome = run_battle(reader, 32, 32, second_star, false)?;
    Ok(outcome as u32)
}

fn generate_map(lines: &[String], max_i: usize, max_j: usize, elf_attack_power: usize) -> Result<Array2<Element>> {
    let mut board: Array2<Element> = Array2::default((max_i, max_j));
    for (j, line) in lines.iter().enumerate() {
        for (i, ch) in line.chars().enumerate() {
            match ch {
                '#' => {
                    board[[i, j]] = Element {
                        kind: ElementKind::Wall,
                        unit: None,
                    }
                }
                '.' => {
                    board[[i, j]] = Element {
                        kind: ElementKind::Cavern,
                        unit: None,
                    }
                }
                'E' => {
                    board[[i, j]] = Element {
                        kind: ElementKind::Unit,
                        unit: Some(Unit::new_elf(elf_attack_power)),
                    }
                }
                'G' => {
                    board[[i, j]] = Element {
                        kind: ElementKind::Unit,
                        unit: Some(Unit::new_goblin()),
                    }
                }
                _ => return Err(anyhow!("invalid game element type!")),
            }
        }
    }

    Ok(board)
}

fn find_enemy_targets(board: &Array2<Element>, unit_kind: UnitKind) -> Vec<[usize; 2]> {
    let units: HashMap<(usize, usize), &Element> = board
        .indexed_iter()
        .filter(|(_idx, element)| element.kind == ElementKind::Unit)
        .filter(|(_idx, element)| if let Some(ref unit) = element.unit { unit.kind != unit_kind } else { false })
        .collect();
    units.keys().map(|(i, j)| [*i, *j]).collect()
}

fn calculate_attack(board: &Array2<Element>, curr_unit: &Unit, coord: [usize; 2], target: &mut Option<[usize; 2]>, min_hit_points: &mut usize) {
    let element = &board[coord];

    if element.kind == ElementKind::Unit {
        if let Some(ref unit) = element.unit {
            if unit.kind != curr_unit.kind {
                let hit_points = unit.hit_points;

                if hit_points < *min_hit_points {
                    *target = Some(coord);
                    *min_hit_points = hit_points;
                }
            }
        }
    }
}

fn attack_adjacent(board: &Array2<Element>, curr_unit: &Unit, i: usize, j: usize, max_i: usize, max_j: usize) -> Option<[usize; 2]> {
    let mut target = None;
    let mut min_hit_points = usize::max_value();

    let above = [i, j - 1];
    let left = [i - 1, j];
    let right = [i + 1, j];
    let down = [i, j + 1];

    // Check up first (reading order and all)
    if j > 0 {
        calculate_attack(board, curr_unit, above, &mut target, &mut min_hit_points);
    }

    if i > 0 {
        calculate_attack(board, curr_unit, left, &mut target, &mut min_hit_points);
    }

    if i < max_i - 1 {
        calculate_attack(board, curr_unit, right, &mut target, &mut min_hit_points);
    }

    if j < max_j - 1 {
        calculate_attack(board, curr_unit, down, &mut target, &mut min_hit_points);
    }
    target
}

#[allow(clippy::cognitive_complexity)]
fn move_if_not_adjacent(
    board: &Array2<Element>,
    targets: &[[usize; 2]],
    curr_unit: &Unit,
    i: usize,
    j: usize,
    max_i: usize,
    max_j: usize,
) -> Result<Option<[usize; 2]>> {
    // If the unit has already moved, don't move again.
    if curr_unit.has_moved {
        return Ok(None);
    }

    // Check for adjacent units, and return if there are any.
    if j > 0 {
        let above = &board[[i, j - 1]];

        if above.kind == ElementKind::Unit {
            if let Some(ref adj_unit) = above.unit {
                if adj_unit.kind != curr_unit.kind {
                    return Ok(None);
                }
            }
        }
    }

    if i > 0 {
        let left = &board[[i - 1, j]];

        if left.kind == ElementKind::Unit {
            if let Some(ref adj_unit) = left.unit {
                if adj_unit.kind != curr_unit.kind {
                    return Ok(None);
                }
            }
        }
    }

    if i < max_i - 1 {
        let right = &board[[i + 1, j]];

        if right.kind == ElementKind::Unit {
            if let Some(ref adj_unit) = right.unit {
                if adj_unit.kind != curr_unit.kind {
                    return Ok(None);
                }
            }
        }
    }

    if j < max_j - 1 {
        let down = &board[[i, j + 1]];

        if down.kind == ElementKind::Unit {
            if let Some(ref adj_unit) = down.unit {
                if adj_unit.kind != curr_unit.kind {
                    return Ok(None);
                }
            }
        }
    }

    // Find the empty spots nearest the targets.
    let actual_locs: Vec<[usize; 2]> = targets
        .iter()
        .flat_map(|x| {
            let i = x[0];
            let j = x[1];
            let mut caverns = Vec::new();
            if j > 0 {
                let above = &board[[i, j - 1]];

                if above.kind == ElementKind::Cavern {
                    caverns.push([i, j - 1]);
                }
            }

            if i > 0 {
                let left = &board[[i - 1, j]];

                if left.kind == ElementKind::Cavern {
                    caverns.push([i - 1, j]);
                }
            }

            if i < max_i - 1 {
                let right = &board[[i + 1, j]];

                if right.kind == ElementKind::Cavern {
                    caverns.push([i + 1, j]);
                }
            }

            if j < max_j - 1 {
                let down = &board[[i, j + 1]];

                if down.kind == ElementKind::Cavern {
                    caverns.push([i, j + 1]);
                }
            }

            caverns
        })
        .collect();

    let mut min_dist = usize::max_value();

    if actual_locs.is_empty() {
        return Ok(None);
    }

    let mut first_step_vec = Vec::new();

    for target in actual_locs {
        let mut visited: Array2<bool> = Array2::default((max_i, max_j));

        Zip::from(&mut visited)
            .and(board)
            .for_each(|visited, element| *visited = matches!(element.kind, ElementKind::Wall | ElementKind::Unit));

        visited[target] = false;

        let mut queue = VecDeque::new();
        let move_queue = VecDeque::new();
        queue.push_back(([i, j], move_queue, 0));

        while !queue.is_empty() {
            let (coord, mut path, dist) = queue.pop_front().ok_or(anyhow!(""))?;

            if coord == target {
                #[allow(clippy::comparison_chain)]
                if dist < min_dist {
                    min_dist = dist;
                    let first_step = path.pop_front().ok_or(anyhow!(""))?;
                    first_step_vec.clear();
                    first_step_vec.push(first_step);
                } else if dist == min_dist {
                    let first_step = path.pop_front().ok_or(anyhow!(""))?;
                    first_step_vec.push(first_step);
                }

                break;
            }

            // Moving up
            let up_coord = [coord[0], coord[1] - 1];
            if j > 0 && !visited[up_coord] {
                let mut new_path = path.clone();
                new_path.push_back(up_coord);
                queue.push_back((up_coord, new_path, dist + 1));
                visited[up_coord] = true;
            }

            // Moving left
            let left_coord = [coord[0] - 1, coord[1]];
            if i > 0 && !visited[left_coord] {
                let mut new_path = path.clone();
                new_path.push_back(left_coord);
                queue.push_back((left_coord, new_path, dist + 1));
                visited[left_coord] = true;
            }

            // Moving right
            let right_coord = [coord[0] + 1, coord[1]];

            if i + 1 < max_i && !visited[right_coord] {
                let mut new_path = path.clone();
                new_path.push_back(right_coord);
                queue.push_back((right_coord, new_path, dist + 1));
                visited[right_coord] = true;
            }

            // Moving down
            let down_coord = [coord[0], coord[1] + 1];

            if j + 1 < max_j && !visited[down_coord] {
                let mut new_path = path.clone();
                new_path.push_back(down_coord);
                queue.push_back((down_coord, new_path, dist + 1));
                visited[down_coord] = true;
            }
        }
    }

    first_step_vec.dedup();

    if !first_step_vec.is_empty() {
        let above = [i, j - 1];
        let left = [i - 1, j];
        let right = [i + 1, j];
        let down = [i, j + 1];

        if first_step_vec.contains(&above) {
            Ok(Some(above))
        } else if first_step_vec.contains(&left) {
            Ok(Some(left))
        } else if first_step_vec.contains(&right) {
            Ok(Some(right))
        } else {
            Ok(Some(down))
        }
    } else {
        Ok(None)
    }
}

fn take_turn(board: &mut Array2<Element>, i: usize, j: usize, max_i: usize, max_j: usize, second_star: bool) -> Result<usize> {
    let mut move_vec = Vec::new();

    // Scope for mutable board change below.
    {
        let curr_cell = &board[[i, j]];

        match curr_cell.kind {
            ElementKind::Wall | ElementKind::Cavern {} => {}
            ElementKind::Unit => {
                if let Some(ref unit) = curr_cell.unit {
                    let targets = find_enemy_targets(board, unit.kind);

                    if targets.is_empty() {
                        return Ok(0);
                    }

                    if let Ok(Some(target)) = move_if_not_adjacent(board, &targets, unit, i, j, max_i, max_j) {
                        move_vec.push((Action::Move([i, j]), target));
                    } else {
                        move_vec.push((Action::No, [0, 0]));
                    }
                }
            }
        }
    }

    let mut next_coord = [0, 0];
    let mut moved = false;

    for (action, coord) in move_vec {
        match action {
            Action::Attack([_, _]) => return Err(anyhow!("Attack in Move Phase")),
            Action::Move([i, j]) => {
                board[coord] = board[[i, j]].clone();
                moved = true;
                next_coord = coord;

                if let Some(ref mut unit) = board[coord].unit {
                    unit.has_moved = true;
                }
                board[[i, j]] = Element {
                    kind: ElementKind::Cavern,
                    unit: None,
                };
            }
            Action::No => {}
        }
    }

    let mut attack_vec = Vec::new();

    {
        let curr_cell = if moved { &board[next_coord] } else { &board[[i, j]] };
        let i = if moved { next_coord[0] } else { i };
        let j = if moved { next_coord[1] } else { j };

        match curr_cell.kind {
            ElementKind::Wall | ElementKind::Cavern {} => {}
            ElementKind::Unit => {
                if let Some(ref unit) = curr_cell.unit {
                    if !unit.has_attacked {
                        if let Some(target) = attack_adjacent(board, unit, i, j, max_i, max_j) {
                            attack_vec.push((Action::Attack([i, j]), target, Some(unit.attack_power)));
                        } else {
                            attack_vec.push((Action::No, [0, 0], None));
                        }
                    }
                }
            }
        }
    }

    for (action, coord, atk_pwr_opt) in attack_vec {
        match action {
            Action::Attack([i, j]) => {
                let mut dead = false;
                if let Some(ref mut unit) = board[[i, j]].unit {
                    unit.has_attacked = true;
                }
                if let Some(ref mut unit) = board[coord].unit {
                    if let Some(atk_pwr) = atk_pwr_opt {
                        unit.hit_points = unit.hit_points.saturating_sub(atk_pwr);

                        if unit.hit_points == 0 {
                            dead = true;

                            if second_star && unit.kind == UnitKind::Elf {
                                return Ok(2);
                            }
                        }
                    }
                }

                if dead {
                    board[coord] = Element {
                        kind: ElementKind::Cavern,
                        unit: None,
                    };
                }
            }
            Action::Move(_) => {}
            Action::No => {}
        }
    }

    Ok(1)
}

fn reset_units(board: &mut Array2<Element>, i: usize, j: usize) {
    let element = &mut board[[i, j]];

    if element.kind == ElementKind::Unit {
        if let Some(ref mut unit) = element.unit {
            unit.has_moved = false;
            unit.has_attacked = false;
        }
    }
}

enum Outcome {
    NoMoreEnemies,
    DeadElf,
    BattleOn,
}

fn round(board: &mut Array2<Element>, i: usize, j: usize, second_star: bool) -> Result<Outcome> {
    let mut done = false;
    let mut dead_elf = false;

    'outer: for row in 0..j {
        for col in 0..i {
            let result = take_turn(board, col, row, i, j, second_star)?;

            if result == 0 {
                done = true;
                break 'outer;
            } else if result == 2 {
                done = true;
                dead_elf = true;
                break 'outer;
            }
        }
    }

    if !done {
        for row in 0..j {
            for col in 0..i {
                reset_units(board, col, row);
            }
        }
    }

    if !done {
        Ok(Outcome::BattleOn)
    } else if !second_star && done {
        Ok(Outcome::NoMoreEnemies)
    } else if second_star && dead_elf {
        Ok(Outcome::DeadElf)
    } else {
        Ok(Outcome::NoMoreEnemies)
    }
}

fn run_battle<T>(reader: T, max_i: usize, max_j: usize, second_star: bool, test: bool) -> Result<usize>
where
    T: BufRead,
{
    let lines: Vec<String> = reader.lines().filter_map(|x| x.ok()).collect();
    let mut board = Array2::default((0, 0));
    let mut dead_elf = true;
    let mut round_count = 0;
    let mut elf_attack_power = 3;

    while dead_elf {
        round_count = 0;
        board = generate_map(&lines, max_i, max_j, elf_attack_power)?;

        let mut done = false;
        while !done {
            match round(&mut board, max_i, max_j, second_star)? {
                Outcome::NoMoreEnemies => {
                    dead_elf = false;
                    done = true;
                }
                Outcome::DeadElf => {
                    if second_star {
                        done = true;
                    }
                }
                _ => {}
            }

            if !done {
                round_count += 1;
            } else if done && test {
                print_board(&board, round_count);
            }
        }
        elf_attack_power += 1;
    }

    let hps: usize = board
        .iter()
        .filter_map(|x| if x.kind == ElementKind::Unit { x.unit.clone() } else { None })
        .map(|u| u.hit_points)
        .sum();
    Ok(round_count * hps)
}

fn print_board(board: &Array2<Element>, round: usize) {
    use std::fmt::Write;

    if round == 0 {
        println!("Initially:");
    } else if round == 1 {
        println!("After 1 round:");
    } else {
        println!("After {round} rounds:");
    }
    for row in board.axis_iter(Axis(1)) {
        let mut unit_vec = Vec::new();
        for cell in row {
            if let Some(ref unit) = cell.unit {
                unit_vec.push((unit.kind, unit.hit_points));
            }
            print!("{cell}");
        }

        let mut buffer = String::new();
        if !unit_vec.is_empty() {
            buffer.push_str("  ");

            for (kind, hitpoints) in unit_vec {
                write!(buffer, "{kind}({hitpoints}), ").expect("");
            }
        }
        let x: &[_] = &[',', ' '];
        print!("{}", buffer.trim_end_matches(x));
        println!();
    }
    println!();
}

#[cfg(test)]
mod one_star {
    use super::run_battle;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_BOARD: &str = r"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";

    const TEST_BOARD_2: &str = r"#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######";

    const TEST_BOARD_3: &str = r"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";

    const TEST_BOARD_4: &str = r"#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";

    const TEST_BOARD_5: &str = r"#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######";

    const TEST_BOARD_6: &str = r"#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(run_battle(Cursor::new(TEST_BOARD), 7, 7, false, true)?, 27730);
        assert_eq!(run_battle(Cursor::new(TEST_BOARD_2), 7, 7, false, true)?, 36334);
        assert_eq!(run_battle(Cursor::new(TEST_BOARD_3), 7, 7, false, true)?, 39514);
        assert_eq!(run_battle(Cursor::new(TEST_BOARD_4), 7, 7, false, true)?, 27755);
        assert_eq!(run_battle(Cursor::new(TEST_BOARD_5), 7, 7, false, true)?, 28944);
        assert_eq!(run_battle(Cursor::new(TEST_BOARD_6), 9, 9, false, true)?, 18740);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::run_battle;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_BOARD: &str = r"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";

    const TEST_BOARD_2: &str = r"#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######";

    const TEST_BOARD_3: &str = r"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";

    const TEST_BOARD_4: &str = r"#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";

    const TEST_BOARD_5: &str = r"#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######";

    const TEST_BOARD_6: &str = r"#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(run_battle(Cursor::new(TEST_BOARD), 7, 7, true, false)?, 4988);
        assert_eq!(run_battle(Cursor::new(TEST_BOARD_2), 7, 7, true, false)?, 29064);
        assert_eq!(run_battle(Cursor::new(TEST_BOARD_3), 7, 7, true, false)?, 31284);
        assert_eq!(run_battle(Cursor::new(TEST_BOARD_4), 7, 7, true, false)?, 3478);
        assert_eq!(run_battle(Cursor::new(TEST_BOARD_5), 7, 7, true, false)?, 6474);
        assert_eq!(run_battle(Cursor::new(TEST_BOARD_6), 9, 9, true, true)?, 1140);
        Ok(())
    }
}
