//! Advent of Code - Day 15 "Beverage Bandits" Solution
use error::Result;
use ndarray::{Array2, Axis};
use std::fmt;
use std::io::BufRead;

enum UnitKind {
    Elf,
    Goblin,
}

struct Unit {
    kind: UnitKind,
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ch = match self.kind {
            UnitKind::Elf => 'E',
            UnitKind::Goblin => 'G',
        };
        write!(f, "{}", ch)
    }
}

enum ElementKind {
    Cavern,
    Wall,
    Unit,
}

impl Default for ElementKind {
    fn default() -> ElementKind {
        ElementKind::Cavern
    }
}

impl fmt::Display for ElementKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ch = match self {
            ElementKind::Cavern => '.',
            ElementKind::Wall => '#',
            ElementKind::Unit => 'U',
        };
        write!(f, "{}", ch)
    }
}

#[derive(Default)]
struct Element {
    kind: ElementKind,
    unit: Option<Unit>,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref u) = self.unit {
            write!(f, "{}", u)
        } else {
            write!(f, "{}", self.kind)
        }
    }
}

pub fn find_solution<T: BufRead>(reader: T, _second_star: bool) -> Result<u32> {
    generate_map(reader, 32, 32)?;
    Ok(0)
}

fn generate_map<T: BufRead>(reader: T, i: usize, j: usize) -> Result<()> {
    let mut board: Array2<Element> = Array2::default((i, j));
    let mut j = 0;
    for line in reader.lines().filter_map(|x| x.ok()) {
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
                        unit: Some(Unit { kind: UnitKind::Elf }),
                    }
                }
                'G' => {
                    board[[i, j]] = Element {
                        kind: ElementKind::Unit,
                        unit: Some(Unit { kind: UnitKind::Goblin }),
                    }
                }
                _ => return Err("invalid game element type!".into()),
            }
        }
        j += 1;
    }

    print_board(&board);
    Ok(())
}

fn print_board(board: &Array2<Element>) {
    for row in board.axis_iter(Axis(1)) {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod one_star {
    use super::generate_map;
    use error::Result;
    use std::io::Cursor;

    const TEST_BOARD: &str = r"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";

    #[test]
    fn solution() -> Result<()> {
        assert!(generate_map(Cursor::new(TEST_BOARD), 7, 7).is_ok());
        Ok(())
    }
}