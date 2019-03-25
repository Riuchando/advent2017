#![feature(extern_prelude)]
use petgraph::prelude::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;

use petgraph as pg;

use petgraph::dot::Dot;

pub fn part1() -> Result<String, Error> {
    let f = File::open("resources/day7.txt")?;
    let f = BufReader::new(f);
    let mut gr = Graph::<_, _>::new();
    let mut node_indicies = HashMap::new();
    let mut node_list = HashMap::new();

    for line in f.lines() {
        // do some parsing
        let line_split = line?
            .parse::<String>()
            .map_err(|_err| Error::new(ErrorKind::InvalidData, format!("couldn't parse")))
            .unwrap();

        let line_split = line_split.split(" -> ").collect::<Vec<&str>>();
        let node = line_split[0].split(" ").collect::<Vec<&str>>()[0];
        let old_node = {
            // don't double add it confuses the graph
            if node_list.contains_key(node) {
                *node_list.get(node).unwrap()
            } else {
                let old_node = gr.add_node(node.to_owned());
                node_list.insert(node.to_owned(), old_node);
                node_indicies.insert(old_node, node.to_owned());
                old_node
            }
        };
        // if this is an edge list, add an edge per item in the incoming list
        if line_split.len() > 1 {
            let edges = line_split[1].split(", ").collect::<Vec<&str>>();
            for edge in edges {
                let new_node = {
                    if node_list.contains_key(edge) {
                        *node_list.get(edge).unwrap()
                    } else {
                        let new_node = gr.add_node(edge.to_owned());
                        node_list.insert(edge.to_owned(), new_node);
                        node_indicies.insert(new_node, edge.to_owned());
                        new_node
                    }
                };

                gr.add_edge(old_node, new_node, 0.0);
            }
        }
    }
    // do the topo sort
    let order = pg::algo::toposort(&gr, None).unwrap();
    // we only care about the first item in the list
    let return_string = node_indicies.get(&order[0]).unwrap();

    // some debugging tool to make sure we parsed this correctly
    println!("{}", Dot::with_config(&gr, &[]));
    Ok(return_string.to_owned())
}
// not worth the effort imo
// pub fn part2() -> Result<String, Error> {
//     let mut return_string = "".to_string();

//     Ok(return_string)
// }
