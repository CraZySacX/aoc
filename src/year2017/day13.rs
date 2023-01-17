//! Advent of Code - Day 13 "Packet Scanners" Solution

use crate::utils::PrivateTryFromUsize;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::io::BufRead;

/// Find the solution for Advent of Code 2017
pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    let mut layer_map = HashMap::new();
    for line_result in reader.lines() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        add_layer_to_map(line, &mut layer_map)?;
    }

    let mut layers: HashMap<usize, Option<u32>> = HashMap::new();
    let maximum_layer = find_maximum_layer(&layer_map)?;
    setup_initial_state(maximum_layer, &layer_map, &mut layers)?;
    let mut result = 0;

    if second_star {
        for i in 0.. {
            let mut layers_to_check = layers.clone();
            if traverse_firewall(&mut layers_to_check, i, true).is_err() {
                continue;
            }
            result = i;
            break;
        }
    } else {
        // Traverse the firewall
        result = traverse_firewall(&mut layers, 0, false)?;
    }

    Ok(result)
}

/// Add a layer to the layer map.
fn add_layer_to_map(line: &str, layer_map: &mut HashMap<usize, u32>) -> Result<()> {
    let layer_desc_vec: Vec<&str> = line.split(": ").collect();
    let layer = layer_desc_vec.first().ok_or(anyhow!("Invalid layer number"))?.parse::<usize>()?;
    let depth = layer_desc_vec.get(1).ok_or(anyhow!("Invalid depty number"))?.parse::<u32>()?;

    layer_map.insert(layer, depth);

    Ok(())
}

/// Find the maximum layer number.
fn find_maximum_layer(layer_map: &HashMap<usize, u32>) -> Result<usize> {
    let max_layer = layer_map.keys().max().ok_or(anyhow!("Unable to find maximum layer"))?;
    Ok(*max_layer)
}

/// Setup the initial state of the layers.
fn setup_initial_state(maximum_layer: usize, layer_map: &HashMap<usize, u32>, layers: &mut HashMap<usize, Option<u32>>) -> Result<()> {
    for i in 0..=maximum_layer {
        if let Some(depth) = layer_map.get(&i) {
            layers.insert(i, Some(*depth));
        } else {
            layers.insert(i, None);
        }
    }
    Ok(())
}

/// Traverse the firewall
fn traverse_firewall(layers: &mut HashMap<usize, Option<u32>>, delay: u32, second_star: bool) -> Result<u32> {
    let mut severity = 0;

    // Loop over layers
    for i in 0..layers.len() {
        let curr_layer = layers.get(&i).ok_or(anyhow!("invalid layer"))?;

        // Get the max depth for this layer. It may be `None`, in which case we will never be
        // caught at this level, so skip.
        if let Some(max_depth) = *curr_layer {
            let current_picosecond = u32::private_try_from(i)?;
            let scan_length = (max_depth - 1) * 2;

            // Each scanner loops back to the 0 level at `((max_depth - 1) * 2)` picoseconds.
            // This means that if `current_picosecond % ((max_depth - 1) * 2) == 0`, then the packet
            // and scanner have met, and the packet is caught.
            if second_star && (current_picosecond + delay) % scan_length == 0 {
                // Uh oh, we got caught.
                return Err(anyhow!("We got caught"));
            } else if !second_star && current_picosecond % scan_length == 0 {
                // Uh oh, we got caught, bump up severity
                severity += current_picosecond * max_depth;
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
        let mut layers: HashMap<usize, Option<u32>> = HashMap::new();
        let maximum_layer = super::find_maximum_layer(&layer_map).expect("");
        assert_eq!(maximum_layer, 6);
        super::setup_initial_state(maximum_layer, &layer_map, &mut layers).expect("");
        assert_eq!(layers.len(), 7);
        assert_eq!(layers.get(&0), Some(&Some(3)));
        assert_eq!(layers.get(&1), Some(&Some(2)));
        assert_eq!(layers.get(&2), Some(&None));
        assert_eq!(layers.get(&3), Some(&None));
        assert_eq!(layers.get(&4), Some(&Some(4)));
        assert_eq!(layers.get(&5), Some(&None));
        assert_eq!(layers.get(&6), Some(&Some(4)));
        let severity = super::traverse_firewall(&mut layers, 0, false).expect("");
        assert_eq!(severity, 24);
    }
}

#[cfg(test)]
mod two_star {
    use std::collections::HashMap;

    #[test]
    fn solution() {
        let mut layer_map = HashMap::new();
        super::add_layer_to_map("0: 3", &mut layer_map).expect("");
        super::add_layer_to_map("1: 2", &mut layer_map).expect("");
        super::add_layer_to_map("4: 4", &mut layer_map).expect("");
        super::add_layer_to_map("6: 4", &mut layer_map).expect("");
        assert_eq!(layer_map.len(), 4);
        let mut layers: HashMap<usize, Option<u32>> = HashMap::new();
        let maximum_layer = super::find_maximum_layer(&layer_map).expect("");
        assert_eq!(maximum_layer, 6);
        super::setup_initial_state(maximum_layer, &layer_map, &mut layers).expect("");
        // assert_eq!(super::traverse_firewall(&mut layers, 0).expect(""), 24);
        // layers.clear();
        // super::setup_initial_state(maximum_layer, &layer_map, &mut layers).expect("");
        // assert_eq!(super::traverse_firewall(&mut layers, 1).expect(""), 2);
        // layers.clear();
        // super::setup_initial_state(maximum_layer, &layer_map, &mut layers).expect("");
        // assert_eq!(super::traverse_firewall(&mut layers, 2).expect(""), 16);
        // layers.clear();
        // super::setup_initial_state(maximum_layer, &layer_map, &mut layers).expect("");
        // assert_eq!(super::traverse_firewall(&mut layers, 3).expect(""), 2);
        // layers.clear();
        // super::setup_initial_state(maximum_layer, &layer_map, &mut layers).expect("");
        // assert_eq!(super::traverse_firewall(&mut layers, 4).expect(""), 0);
        // assert_eq!(super::traverse_firewall(&mut layers, 5).expect(""), 2);
        // assert_eq!(super::traverse_firewall(&mut layers, 6).expect(""), 0);
        // assert_eq!(super::traverse_firewall(&mut layers, 7).expect(""), 0);
        // assert_eq!(super::traverse_firewall(&mut layers, 8).expect(""), 0);
        // assert_eq!(super::traverse_firewall(&mut layers, 9).expect(""), 0);
        // layers.clear();
        // super::setup_initial_state(maximum_layer, &layer_map, &mut layers).expect("");
        assert_eq!(super::traverse_firewall(&mut layers, 10, true).expect(""), 0);
    }
}
