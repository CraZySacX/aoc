//! Advent of Code - Day 13 Solution
use error::Result;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::BufRead;

/// Represents scanner direction.
enum Direction {
    /// The scanner is moving down in depth.
    Down,
    /// The scanner is moving up in depth.
    Up,
}

/// Represents a firewall layer.
struct Layer {
    /// Is the packet present in the current layer.
    packet_present: bool,
    /// The current scanner depth at this layer.
    scanner_depth: Option<u32>,
    /// The maximum scanner depth at this layer.
    scanner_max_depth: Option<u32>,
    /// The current scanner direction.
    direction: Direction,
}

/// Find the solution for Advent of Code 2017
pub fn find_solution<T: BufRead>(reader: T, _second_star: bool) -> Result<u32> {
    let mut layer_map = HashMap::new();
    for line_result in reader.lines() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        add_layer_to_map(line, &mut layer_map)?;
    }

    // Setup initial state
    let maximum_layer = find_maximum_layer(&layer_map)?;
    let mut layers: HashMap<usize, Layer> = HashMap::new();
    setup_initial_state(maximum_layer, &layer_map, &mut layers)?;

    // Traverse the firewall
    let severity = traverse_firewall(&mut layers)?;

    Ok(severity)
}

/// Add a layer to the layer map.
fn add_layer_to_map(line: &str, layer_map: &mut HashMap<usize, u32>) -> Result<()> {
    let layer_desc_vec: Vec<&str> = line.split(": ").collect();
    let layer = layer_desc_vec
        .get(0)
        .ok_or("Invalid layer number")?
        .parse::<usize>()?;
    let depth = layer_desc_vec
        .get(1)
        .ok_or("Invalid depty number")?
        .parse::<u32>()?;

    layer_map.insert(layer, depth);

    Ok(())
}

/// Find the maximum layer number.
fn find_maximum_layer(layer_map: &HashMap<usize, u32>) -> Result<usize> {
    let max_layer = layer_map
        .keys()
        .max()
        .ok_or("Unable to find maximum layer")?;
    Ok(*max_layer)
}

/// Setup the initial state of the layers.
fn setup_initial_state(maximum_layer: usize, layer_map: &HashMap<usize, u32>, layers: &mut HashMap<usize, Layer>) -> Result<()> {
    for i in 0..=maximum_layer {
        if let Some(depth) = layer_map.get(&i) {
            layers.insert(
                i,
                Layer {
                    packet_present: false,
                    scanner_depth: Some(0),
                    scanner_max_depth: Some(*depth),
                    direction: Direction::Down,
                },
            );
        } else {
            layers.insert(
                i,
                Layer {
                    packet_present: false,
                    scanner_depth: None,
                    scanner_max_depth: None,
                    direction: Direction::Down,
                },
            );
        }
    }
    Ok(())
}

/// Traverse the firewall
fn traverse_firewall(layers: &mut HashMap<usize, Layer>) -> Result<u32> {
    let mut severity = 0;
    let total_layers = layers.len();

    // Loop over layers
    for i in 0..total_layers {
        {
            // Move packet
            let curr_layer = layers.get_mut(&i).ok_or("invalid layer")?;
            curr_layer.packet_present = true;

            // Calculate severity
            if let Some(depth) = curr_layer.scanner_depth {
                if depth == 0 {
                    let max_depth = curr_layer.scanner_max_depth.ok_or("invalid max depth")?;
                    let layer_u32: u32 = TryFrom::try_from(i)?;
                    severity += layer_u32 * max_depth;
                }
            }
        }
        // Loop over layers again, moving scanners.
        for (_, v) in layers.iter_mut() {
            if let Some(curr_depth) = v.scanner_depth {
                let max_depth = v.scanner_max_depth.ok_or("invalid max depth")?;

                match v.direction {
                    Direction::Down => {
                        if curr_depth == max_depth - 1 {
                            v.direction = Direction::Up;
                            v.scanner_depth = Some(curr_depth - 1);
                        } else {
                            v.scanner_depth = Some(curr_depth + 1);
                        }
                    }
                    Direction::Up => {
                        if curr_depth == 0 {
                            v.direction = Direction::Down;
                            v.scanner_depth = Some(curr_depth + 1);
                        } else {
                            v.scanner_depth = Some(curr_depth - 1);
                        }
                    }
                }
            }
        }
    }

    Ok(severity)
}
#[cfg(test)]
mod one_star {
    use std::collections::HashMap;

    #[test]
    fn solution() {
        let mut layer_map = HashMap::new();
        super::add_layer_to_map("0: 3", &mut layer_map).expect("");
        super::add_layer_to_map("1: 2", &mut layer_map).expect("");
        super::add_layer_to_map("4: 4", &mut layer_map).expect("");
        super::add_layer_to_map("6: 4", &mut layer_map).expect("");
        assert_eq!(layer_map.len(), 4);
        let mut layers: HashMap<usize, super::Layer> = HashMap::new();
        let maximum_layer = super::find_maximum_layer(&layer_map).expect("");
        assert_eq!(maximum_layer, 6);
        super::setup_initial_state(maximum_layer, &layer_map, &mut layers).expect("");
        assert_eq!(layers.len(), 7);
        if let Some(layer_0) = layers.get(&0) {
            assert!(!layer_0.packet_present);
            assert_eq!(layer_0.scanner_depth, Some(0));
            assert_eq!(layer_0.scanner_max_depth, Some(3));
        } else {
            assert!(false);
        }
        if let Some(layer_2) = layers.get(&2) {
            assert!(!layer_2.packet_present);
            assert_eq!(layer_2.scanner_depth, None);
            assert_eq!(layer_2.scanner_max_depth, None);
        } else {
            assert!(false);
        }
        let severity = super::traverse_firewall(&mut layers).expect("");
        assert_eq!(severity, 24);
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {
        assert!(true);
    }
}
