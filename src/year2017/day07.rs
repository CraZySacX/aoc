//! Advent of Code - Day 7 "Recursive Circus" Solution
use error::Result;
use std::collections::HashMap;
use std::io::BufRead;

/// Day 7 Node
#[derive(Clone)]
struct Node {
    /// Node ID
    id: usize,
    /// Node Name
    name: String,
    /// Node Weight
    #[allow(dead_code)]
    weight: u32,
    /// Node Parent
    parent: Option<usize>,
    /// Node Children
    #[allow(dead_code)]
    children: Option<Vec<usize>>,
}

/// Parse the file at `filename` and generate the checksum.
pub fn find_solution<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    use std::io::{self, Write};
    let mut nodes: Vec<Node> = Vec::new();
    let mut children: HashMap<usize, Vec<String>> = HashMap::new();
    for (id, line_result) in reader.lines().enumerate() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        parse_line(line, id, &mut nodes, &mut children)?;
    }

    assign_parents(&mut nodes, &mut children)?;

    if second_star {
        let idx = find_root(&nodes)?;
        assign_children(&mut nodes, &children)?;
        let root = nodes.get(idx).ok_or("Invalid root node")?;
        let mut curr_weights = children_weight(&nodes, root).expect("");
        let mut curr_tuple = are_my_children_balanced(&curr_weights, 0).expect("");
        let mut is_balanced = curr_tuple.2;

        while !is_balanced {
            let node = nodes.get(curr_tuple.0).ok_or("").expect("");
            curr_weights = children_weight(&nodes, node).expect("");
            curr_tuple = are_my_children_balanced(&curr_weights, curr_tuple.1).expect("");
            is_balanced = curr_tuple.2
        }
        Ok(curr_tuple.1)
    } else {
        let idx = find_root(&nodes)?;
        let node = nodes.get(idx).ok_or("Not a good index")?;
        writeln!(io::stdout(), "Node {}: {}", node.id, node.name)?;
        Ok(0)
    }
}

/// Parse a node description line, and add the nodes and children to the appropriate structures.
fn parse_line(line: &str, id: usize, nodes: &mut Vec<Node>, children: &mut HashMap<usize, Vec<String>>) -> Result<()> {
    let node_def: Vec<&str> = line.split(" -> ").collect();

    let node_desc = node_def.get(0).ok_or("Unable to get node description")?;
    let desc: Vec<&str> = node_desc.split(' ').collect();
    let name = desc.get(0).ok_or("Unable to deternmine node name")?;
    let weight_str = desc.get(1).ok_or("Unable to determine node weight")?;
    let weight = weight_str.trim_matches(|c| c == '(' || c == ')').parse::<u32>()?;

    if let Some(children_desc) = node_def.get(1) {
        let children_vec: Vec<String> = children_desc.split(", ").map(String::from).collect();
        children.insert(id, children_vec);
    }

    nodes.push(Node {
        id,
        name: String::from(*name),
        weight,
        parent: None,
        children: None,
    });

    Ok(())
}

/// Assign parent nodes for children
fn assign_parents(nodes: &mut Vec<Node>, children: &mut HashMap<usize, Vec<String>>) -> Result<()> {
    for (k, v) in children {
        for child_name in v {
            for node in nodes.iter_mut() {
                if &node.name == child_name {
                    node.parent = Some(*k);
                }
            }
        }
    }
    Ok(())
}

/// Find the root of the tree
fn find_root(nodes: &[Node]) -> Result<usize> {
    let mut id: usize = 0;
    let mut count = 0;

    for node in nodes {
        if node.parent.is_none() {
            id = node.id;
            count += 1;
        }
    }

    if count == 1 {
        Ok(id)
    } else if count == 0 {
        Err("No root found".into())
    } else {
        Err("Too many roots found".into())
    }
}

/// Traverse the vectors, assigning proper children vectors.
fn assign_children(nodes: &mut Vec<Node>, children: &HashMap<usize, Vec<String>>) -> Result<()> {
    let mut child_ids: HashMap<usize, Vec<usize>> = HashMap::new();

    for (k, v) in children {
        let mut child_id_vec = Vec::new();

        for child_name in v {
            for node in nodes.iter() {
                if &node.name == child_name {
                    child_id_vec.push(node.id);
                }
            }
        }

        child_ids.insert(*k, child_id_vec);
    }

    for (k, v) in child_ids {
        let mut node = nodes.get_mut(k).ok_or("Invalid node id")?;
        node.children = Some(v);
    }

    Ok(())
}

/// Traverse the tree accumulating weights
fn children_weight(nodes: &[Node], start_node: &Node) -> Result<Vec<(usize, u32)>> {
    let mut weights = Vec::new();
    if start_node.children.is_none() {
        weights.push((start_node.id, start_node.weight));
    } else {
        let node_clone = (*start_node).clone();
        let children = node_clone.children.ok_or("Bad children")?;

        for node_id in children {
            let node = nodes.get(node_id).ok_or("Cannot find child node")?;
            let total_child_weight: u32 = children_weight(nodes, node)?.iter().fold(0, |acc, x| acc + x.1);
            weights.push((node_id, total_child_weight));
        }

        weights.push((node_clone.id, node_clone.weight));
    }
    Ok(weights)
}

/// Check weights
fn are_my_children_balanced(outer: &[(usize, u32)], diff: u32) -> Result<(usize, u32, bool)> {
    let len = outer.len() - 1;
    let mut inner = vec![(0, 0); outer.len()];
    inner.copy_from_slice(outer);

    for (i, x) in outer.iter().enumerate().take(len) {
        for y in inner.iter().take(len).skip(i + 1) {
            let outer_id = x.0;
            let inner_id = y.0;
            let ov = x.1;
            let iv = y.1;

            if ov == iv {
                continue;
            }
            if ov < iv {
                return Ok((inner_id, (iv - ov), false));
            } else {
                return Ok((outer_id, (ov - iv), false));
            }
        }
    }
    Ok((outer[len].0, outer[len].1 - diff, true))
}

#[cfg(test)]
fn setup_tree(nodes: &mut Vec<Node>, children: &mut HashMap<usize, Vec<String>>) -> Result<()> {
    parse_line("pbga (66)", 0, nodes, children)?;
    parse_line("xhth (57)", 1, nodes, children)?;
    parse_line("ebii (61)", 2, nodes, children)?;
    parse_line("havc (66)", 3, nodes, children)?;
    parse_line("ktlj (57)", 4, nodes, children)?;
    parse_line("fwft (72) -> ktlj, cntj, xhth", 5, nodes, children)?;
    parse_line("qoyq (66)", 6, nodes, children)?;
    parse_line("padx (45) -> pbga, havc, qoyq", 7, nodes, children)?;
    parse_line("tknk (41) -> ugml, padx, fwft", 8, nodes, children)?;
    parse_line("jptl (61)", 9, nodes, children)?;
    parse_line("ugml (68) -> gyxo, ebii, jptl", 10, nodes, children)?;
    parse_line("gyxo (61)", 11, nodes, children)?;
    parse_line("cntj (57)", 12, nodes, children)?;
    assign_parents(nodes, children)?;
    Ok(())
}

#[cfg(test)]
mod one_star {
    use std::collections::HashMap;

    #[test]
    fn solution() {
        let mut nodes: Vec<super::Node> = Vec::new();
        let mut children: HashMap<usize, Vec<String>> = HashMap::new();
        super::setup_tree(&mut nodes, &mut children).expect("Unable to setup tree");
        assert_eq!(nodes.len(), 13);
        assert_eq!(children.len(), 4);
        let mut keys_vec: Vec<usize> = children.keys().cloned().collect();
        keys_vec.sort();
        assert_eq!(keys_vec, vec![5, 7, 8, 10]);
        super::assign_parents(&mut nodes, &mut children).expect("");
        assert_eq!(super::find_root(&nodes).unwrap_or(0), 8);
    }
}

#[cfg(test)]
mod two_star {
    use std::collections::HashMap;

    #[test]
    fn solution() {
        let mut nodes: Vec<super::Node> = Vec::new();
        let mut children: HashMap<usize, Vec<String>> = HashMap::new();
        super::setup_tree(&mut nodes, &mut children).expect("Unable to setup tree");
        super::assign_children(&mut nodes, &mut children).expect("Unable to assign children");
        let node = nodes.get(8).ok_or("").expect("");
        let mut curr_weights = super::children_weight(&nodes, node).expect("");
        assert_eq!(curr_weights, vec![(10, 251), (7, 243), (5, 243), (8, 41)]);
        let mut curr_tuple = super::are_my_children_balanced(&curr_weights, 0).expect("");
        assert_eq!(curr_tuple, (10, 8, false));
        let mut is_balanced = curr_tuple.2;

        while !is_balanced {
            let node = nodes.get(curr_tuple.0).ok_or("").expect("");
            curr_weights = super::children_weight(&nodes, node).expect("");
            assert_eq!(curr_weights, vec![(11, 61), (2, 61), (9, 61), (10, 68)]);
            curr_tuple = super::are_my_children_balanced(&curr_weights, curr_tuple.1).expect("");
            assert_eq!(curr_tuple, (10, 60, true));
            is_balanced = curr_tuple.2
        }

        assert_eq!(curr_tuple.1, 60);
    }
}
