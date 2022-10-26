//! Advent of Code - Day 13 "Mine Cart Madness" Solution
use error::Result;
use ndarray::{Array2, Axis};
use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::BTreeMap;
use std::fmt;
use std::io::BufRead;

enum TrackKind {
    UpDown,
    LeftRight,
    CurveRight,
    CurveLeft,
    Junction,
    Empty,
    Collision,
}

impl Default for TrackKind {
    fn default() -> TrackKind {
        TrackKind::Empty
    }
}

impl fmt::Display for TrackKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ch = match self {
            TrackKind::UpDown => '|',
            TrackKind::LeftRight => '-',
            TrackKind::CurveRight => '/',
            TrackKind::CurveLeft => '\\',
            TrackKind::Junction => '+',
            TrackKind::Empty => ' ',
            TrackKind::Collision => 'X',
        };
        write!(f, "{ch}")
    }
}

#[derive(Default, Getters)]
struct Track {
    kind: TrackKind,
    #[get]
    cart: Option<Cart>,
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref c) = self.cart {
            write!(f, "{c}")
        } else {
            write!(f, "{}", self.kind)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq)]
struct CartPoint {
    i: usize,
    j: usize,
}

impl PartialOrd for CartPoint {
    fn partial_cmp(&self, other: &CartPoint) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CartPoint {
    fn cmp(&self, other: &CartPoint) -> Ordering {
        if self.j < other.j {
            Ordering::Less
        } else if self.j > other.j {
            Ordering::Greater
        } else if self.i < other.i {
            Ordering::Less
        } else if self.i > other.i {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialEq for CartPoint {
    fn eq(&self, other: &CartPoint) -> bool {
        self.i == other.i && self.j == other.j
    }
}

#[derive(Clone, Copy, Debug)]
enum CartTurnState {
    Left,
    Straight,
    Right,
}

#[derive(Clone, Copy, Debug)]
enum CartDirection {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for CartDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ch = match self {
            CartDirection::Up => '^',
            CartDirection::Down => 'v',
            CartDirection::Left => '<',
            CartDirection::Right => '>',
        };
        write!(f, "{ch}")
    }
}

#[derive(Debug)]
struct Cart {
    direction: CartDirection,
    turn_state: CartTurnState,
}

impl fmt::Display for Cart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.direction)
    }
}

pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    if let Some((i, j)) = run_carts(reader, 150, 150, second_star, false)? {
        println!("Result: {i},{j}");
    }

    Ok(0)
}

fn gen_mine<T: BufRead>(reader: T, i: usize, j: usize) -> Result<Array2<Track>> {
    let mut mine_arr: Array2<Track> = Array2::default((i, j));
    for (j, line) in reader.lines().filter_map(|x| x.ok()).enumerate() {
        for (i, ch) in line.chars().enumerate() {
            let (kind, cart) = match ch {
                '/' => (TrackKind::CurveRight, None),
                '\\' => (TrackKind::CurveLeft, None),
                '|' => (TrackKind::UpDown, None),
                '-' => (TrackKind::LeftRight, None),
                '+' => (TrackKind::Junction, None),
                '^' => (
                    TrackKind::UpDown,
                    Some(Cart {
                        direction: CartDirection::Up,
                        turn_state: CartTurnState::Left,
                    }),
                ),
                'v' => (
                    TrackKind::UpDown,
                    Some(Cart {
                        direction: CartDirection::Down,
                        turn_state: CartTurnState::Left,
                    }),
                ),
                '<' => (
                    TrackKind::LeftRight,
                    Some(Cart {
                        direction: CartDirection::Left,
                        turn_state: CartTurnState::Left,
                    }),
                ),
                '>' => (
                    TrackKind::LeftRight,
                    Some(Cart {
                        direction: CartDirection::Right,
                        turn_state: CartTurnState::Left,
                    }),
                ),
                _ => (TrackKind::Empty, None),
            };

            mine_arr[[i, j]] = Track { kind, cart };
        }
    }

    Ok(mine_arr)
}

fn run_carts<T: BufRead>(reader: T, i: usize, j: usize, second_star: bool, test: bool) -> Result<Option<(usize, usize)>> {
    if test {
        println!();
    }
    let mut mine_arr = gen_mine(reader, i, j)?;
    if test {
        print_mine_arr(&mine_arr);
    }

    let mut res;
    loop {
        let mut cart_map = find_carts(&mine_arr);
        res = move_carts(&mut cart_map, &mut mine_arr, second_star)?;

        if test {
            print_mine_arr(&mine_arr);
        }

        if res.is_some() {
            break;
        }
    }

    Ok(res)
}

fn find_carts(mine_arr: &Array2<Track>) -> BTreeMap<CartPoint, CartDirection> {
    let mut cart_map = BTreeMap::new();
    for (idx, t) in mine_arr.indexed_iter() {
        if let Some(ref c) = t.cart {
            cart_map.insert(CartPoint { i: idx.0, j: idx.1 }, c.direction);
        }
    }
    cart_map
}

fn move_carts(cart_map: &mut BTreeMap<CartPoint, CartDirection>, mine_arr: &mut Array2<Track>, second_star: bool) -> Result<Option<(usize, usize)>> {
    for (cart_point, direction) in cart_map.iter() {
        let i = cart_point.i;
        let j = cart_point.j;

        // Save off the turn state in case of junction.
        let turn_state = if let Some(curr_cart) = mine_arr[[i, j]].cart() {
            curr_cart.turn_state
        } else {
            continue;
        };

        // Remove the cart from the old track.
        {
            let mut track = &mut mine_arr[[i, j]];
            track.cart = None;
        }

        // Generate the next index
        let nidx = match direction {
            CartDirection::Down => [i, j + 1],
            CartDirection::Up => [i, j - 1],
            CartDirection::Right => [i + 1, j],
            CartDirection::Left => [i - 1, j],
        };

        let mut collision = false;
        if second_star {
            // Get the track at the next index
            let track = &mine_arr[nidx];

            // If there is already a cart there, COLLISION!
            if track.cart().is_some() {
                collision = true;
            }
        } else {
            // Get the track at the next index
            let track = &mut mine_arr[nidx];

            // If there is already a cart there, COLLISION!
            // Set the track appropriately for display
            // and return the index where the collision happened.
            if track.cart().is_some() {
                track.kind = TrackKind::Collision;
                track.cart = None;
                return Ok(Some((nidx[0], nidx[1])));
            }
        }

        if collision {
            {
                let mut track = &mut mine_arr[nidx];
                if track.cart().is_some() {
                    track.cart = None;
                }
            }

            let carts = find_carts(mine_arr);
            if carts.len() == 1 {
                let (cart_point, direction) = carts.iter().next().ok_or("")?;
                let i = cart_point.i;
                let j = cart_point.j;

                let fidx = match direction {
                    CartDirection::Up => [i, j - 1],
                    CartDirection::Down => [i, j + 1],
                    CartDirection::Left => [i - 1, j],
                    CartDirection::Right => [i + 1, j],
                };
                return Ok(Some((fidx[0], fidx[1])));
            }
        } else {
            let mut track = &mut mine_arr[nidx];

            // Otherwise, setup the new track position.
            match track.kind {
                TrackKind::Junction => {
                    let new_direction = match turn_state {
                        CartTurnState::Left => match direction {
                            CartDirection::Down => CartDirection::Right,
                            CartDirection::Up => CartDirection::Left,
                            CartDirection::Right => CartDirection::Up,
                            CartDirection::Left => CartDirection::Down,
                        },
                        CartTurnState::Straight => *direction,
                        CartTurnState::Right => match direction {
                            CartDirection::Down => CartDirection::Left,
                            CartDirection::Up => CartDirection::Right,
                            CartDirection::Right => CartDirection::Down,
                            CartDirection::Left => CartDirection::Up,
                        },
                    };

                    track.cart = Some(Cart {
                        direction: new_direction,
                        turn_state: next_turn_state(turn_state),
                    });
                }
                TrackKind::UpDown | TrackKind::LeftRight => {
                    track.cart = Some(Cart {
                        direction: *direction,
                        turn_state,
                    });
                }
                TrackKind::CurveLeft => {
                    let new_direction = match direction {
                        CartDirection::Down => CartDirection::Right,
                        CartDirection::Up => CartDirection::Left,
                        CartDirection::Right => CartDirection::Down,
                        CartDirection::Left => CartDirection::Up,
                    };
                    track.cart = Some(Cart {
                        direction: new_direction,
                        turn_state,
                    });
                }
                TrackKind::CurveRight => {
                    let new_direction = match direction {
                        CartDirection::Down => CartDirection::Left,
                        CartDirection::Up => CartDirection::Right,
                        CartDirection::Right => CartDirection::Up,
                        CartDirection::Left => CartDirection::Down,
                    };
                    track.cart = Some(Cart {
                        direction: new_direction,
                        turn_state,
                    });
                }
                TrackKind::Collision => return Err("Can't move into a collision area!".into()),
                TrackKind::Empty => return Err("Can't move into an empty area".into()),
            }
        }
    }
    Ok(None)
}

/// Cycle the turn state machine
fn next_turn_state(turn_state: CartTurnState) -> CartTurnState {
    match turn_state {
        CartTurnState::Left => CartTurnState::Straight,
        CartTurnState::Straight => CartTurnState::Right,
        CartTurnState::Right => CartTurnState::Left,
    }
}

fn print_mine_arr(mine_arr: &Array2<Track>) {
    for row in mine_arr.axis_iter(Axis(1)) {
        for cell in row {
            print!("{cell}");
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod one_star {
    use super::run_carts;
    use error::Result;
    use std::io::Cursor;

    const TEST_STATE: &str = r"|
v
|
|
|
^
|";

    const TEST_STATE_2: &str = r"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   ";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(run_carts(Cursor::new(TEST_STATE), 1, 7, false, true)?, Some((0, 3)));
        Ok(())
    }

    #[test]
    fn solution2() -> Result<()> {
        assert_eq!(run_carts(Cursor::new(TEST_STATE_2), 13, 6, false, true)?, Some((7, 3)));
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::run_carts;
    use error::Result;
    use std::io::Cursor;

    const TEST_STATE: &str = r"/>-<\
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(run_carts(Cursor::new(TEST_STATE), 7, 7, true, true)?, Some((6, 4)));
        Ok(())
    }
}
