extern crate petgraph;


use std::fmt;

use petgraph::graph::{Graph, NodeIndex};


#[derive(PartialEq, Eq, Debug)]
pub enum Symbol {
    Down,
    Split,
    Last,
    Blank,
}
impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Symbol::Down => write!(f, "│   "),
            Symbol::Split => write!(f, "├──"),
            Symbol::Last => write!(f, "└──"),
            Symbol::Blank => write!(f, "   "),
        }
    }
}


pub fn print(graph: &Graph<&str, &str>, root: NodeIndex, print_root: bool) {
    if print_root {
        print_tree(graph, root, &mut Vec::new());
    } else {
        for neighbor in graph.neighbors(root) {
            print_tree(graph, neighbor, &mut Vec::new());
        }
    }
}


pub fn print_tree<N, E>(graph: &Graph<N, E>, node: NodeIndex, levels: &mut Vec<Symbol>)
    where N: fmt::Display,
          E: fmt::Display
{
    let mut prefix = levels.iter()
        .fold(String::new(), |acc, ref s| acc + &format!("{}", &s));
    if !prefix.is_empty() {
        prefix.push_str(" ");
    }

    if !levels.is_empty() {
        if levels.last().unwrap() == &Symbol::Last {
            levels.pop();
            levels.push(Symbol::Blank);
        } else {
            levels.pop();
            levels.push(Symbol::Down);
        }
    }
    println!("{}{}", prefix, graph[node]);

    let mut neighbors: Vec<NodeIndex> = graph.neighbors(node).collect();
    let last = neighbors.pop();
    for neighbor in neighbors {
        levels.push(Symbol::Split);
        print_tree(graph, neighbor, levels);
    }

    if let Some(l) = last {
        let l_neighbors: Vec<NodeIndex> = graph.neighbors(l).collect();
        if l_neighbors.len() <= 1 {
            levels.push(Symbol::Last);
        } else {
            levels.push(Symbol::Split);
        }
        print_tree(graph, l, levels);
    }

    levels.pop();
}
