extern crate petgraph;


use std::fmt;

use petgraph::graph as pgraph;


pub trait Tree<T, K>
    where K: std::fmt::Display + std::clone::Clone
{
    type Index;
    fn nodes(&self, index: &Self::Index) -> Vec<Self::Index>;
    fn get(&self, index: &Self::Index) -> K;
}

impl<K, E> Tree<pgraph::Graph<K, E>, K> for pgraph::Graph<K, E>
    where K: fmt::Display + std::clone::Clone
{
    type Index = pgraph::NodeIndex;
    fn nodes(&self, index: &Self::Index) -> Vec<Self::Index> {
        self.neighbors(*index).collect()
    }

    fn get(&self, index: &Self::Index) -> K {
        self[*index].clone()
    }
}

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


pub fn print<T: Tree<G, K, Index = I>, G, K, I>(graph: &T, root: I, print_root: bool)
    where K: fmt::Display + std::clone::Clone
{
    if print_root {
        print_tree::<T, G, K, I>(graph, root, &mut Vec::new());
    } else {
        for neighbor in graph.nodes(&root) {
            print_tree::<T, G, K, I>(graph, neighbor, &mut Vec::new());
        }
    }
}


pub fn print_tree<T: Tree<G, K, Index = I>, G, K, I>(graph: &T, node: I, levels: &mut Vec<Symbol>)
    where K: fmt::Display + std::clone::Clone
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
    println!("{}{}", prefix, graph.get(&node));

    let mut neighbors: Vec<I> = graph.nodes(&node);
    let last = neighbors.pop();

    for neighbor in neighbors {
        levels.push(Symbol::Split);
        print_tree(graph, neighbor, levels);
    }

    if let Some(l) = last {
        if graph.nodes(&l).len() <= 1 {
            levels.push(Symbol::Last);
        } else {
            levels.push(Symbol::Split);
        }
        print_tree(graph, l, levels);
    }

    levels.pop();
}
