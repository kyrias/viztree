extern crate petgraph;

extern crate viztree;


use petgraph::Graph;


fn main() {
    let mut graph = Graph::<&str, &str>::new();
    let root = graph.add_node("root");

    let a = graph.add_node("a");
    let b = graph.add_node("b");
    let c = graph.add_node("c");

    let a_a = graph.add_node("a_a");
    let a_b = graph.add_node("a_b");
    let a_c = graph.add_node("a_c");

    let b_a = graph.add_node("b_a");
    let b_b = graph.add_node("b_b");
    let b_c = graph.add_node("b_c");

    let a_b_a = graph.add_node("a_b_a");
    let a_b_b = graph.add_node("a_b_b");
    let a_b_c = graph.add_node("a_b_c");

    let b_a_a = graph.add_node("b_a_a");
    let b_a_b = graph.add_node("b_a_b");

    graph.extend_with_edges(&[(b_a, b_a_b),
                              (b_a, b_a_a),

                              (a_b, a_b_c),
                              (a_b, a_b_b),
                              (a_b, a_b_a),

                              (b, b_c),
                              (b, b_b),
                              (b, b_a),

                              (a, a_c),
                              (a, a_b),
                              (a, a_a),

                              (root, c),
                              (root, b),
                              (root, a)]);

    viztree::print(&graph, root, true);
}
