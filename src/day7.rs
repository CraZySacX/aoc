//! Advent of Code - Day 7 Solution
use error::Result;
use std::collections::HashMap;
use std::io::BufRead;

/// Day 7 Node
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
pub fn build_tree<T: BufRead>(reader: T, second_star: bool) -> Result<u32> {
    use std::io::{self, Write};
    let mut nodes: Vec<Node> = Vec::new();
    let mut children: HashMap<usize, Vec<String>> = HashMap::new();
    for (id, line_result) in reader.lines().enumerate() {
        let line = &line_result.unwrap_or_else(|_| "".to_string());
        parse_line(line, id, &mut nodes, &mut children)?;
    }

    assign_parents(&mut nodes, &mut children)?;

    if second_star {
        Ok(0)
    } else {
        let idx = find_root(&nodes)?;
        let node = nodes.get(idx).ok_or("Not a good index")?;
        writeln!(io::stdout(), "Node {}: {}", node.id, node.name)?;
        Ok(0)
    }
}

/// Parse a node description line, and add the nodes and children to the appropriate structures.
fn parse_line(
    line: &str,
    id: usize,
    nodes: &mut Vec<Node>,
    children: &mut HashMap<usize, Vec<String>>,
) -> Result<()> {
    let node_def: Vec<&str> = line.split(" -> ").collect();

    let node_desc = node_def.get(0).ok_or("Unable to get node description")?;
    let desc: Vec<&str> = node_desc.split(' ').collect();
    let name = desc.get(0).ok_or("Unable to deternmine node name")?;
    let weight_str = desc.get(1).ok_or("Unable to determine node weight")?;
    let weight = weight_str
        .trim_matches(|c| c == '(' || c == ')')
        .parse::<u32>()?;

    if let Some(children_desc) = node_def.get(1) {
        let children_vec: Vec<String> = children_desc.split(", ").map(String::from).collect();
        children.insert(id, children_vec);
    }

    nodes.push(Node {
        id: id,
        name: String::from(*name),
        weight: weight,
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

#[cfg(test)]
mod test {
    use super::{assign_parents, find_root, parse_line, Node};
    use std::collections::HashMap;

    #[test]
    fn build_a_tree() {
        let mut nodes: Vec<Node> = Vec::new();
        let mut children: HashMap<usize, Vec<String>> = HashMap::new();
        parse_line("pbga (66)", 0, &mut nodes, &mut children).expect("");
        parse_line("xhth (57)", 1, &mut nodes, &mut children).expect("");
        parse_line("ebii (61)", 2, &mut nodes, &mut children).expect("");
        parse_line("havc (66)", 3, &mut nodes, &mut children).expect("");
        parse_line("ktlj (57)", 4, &mut nodes, &mut children).expect("");
        parse_line(
            "fwft (72) -> ktlj, cntj, xhth",
            5,
            &mut nodes,
            &mut children,
        ).expect("");
        parse_line("qoyq (66)", 6, &mut nodes, &mut children).expect("");
        parse_line(
            "padx (45) -> pbga, havc, qoyq",
            7,
            &mut nodes,
            &mut children,
        ).expect("");
        parse_line(
            "tknk (41) -> ugml, padx, fwft",
            8,
            &mut nodes,
            &mut children,
        ).expect("");
        parse_line("jptl (61)", 9, &mut nodes, &mut children).expect("");
        parse_line(
            "ugml (68) -> gyxo, ebii, jptl",
            10,
            &mut nodes,
            &mut children,
        ).expect("");
        parse_line("gyxo (61)", 11, &mut nodes, &mut children).expect("");
        parse_line("cntj (57)", 12, &mut nodes, &mut children).expect("");
        assert_eq!(nodes.len(), 13);
        assert_eq!(children.len(), 4);
        let mut keys_vec: Vec<usize> = children.keys().cloned().collect();
        keys_vec.sort();
        assert_eq!(keys_vec, vec![5, 7, 8, 10]);
        assign_parents(&mut nodes, &mut children).expect("");
        assert_eq!(find_root(&nodes).unwrap_or(0), 8);
    }
}
