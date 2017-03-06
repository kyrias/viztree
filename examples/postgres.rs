//! Tree structure stolen from
//! https://github.com/sfackler/cargo-tree/blob/1916fde82cb3fe6f0471b0b81f8aa0037f48dc7c/README.md

extern crate petgraph;
extern crate viztree;


use petgraph::Graph;


fn main() {
    let mut graph = Graph::<&str, &str>::new();
    let postgres = graph.add_node("postgres");

    let bufstream = graph.add_node("bufstream v0.1.1");
    let byteorder = graph.add_node("byteorder v0.4.2");
    let hex = graph.add_node("hex v0.1.0");
    let log = graph.add_node("log v0.3.4");
    let libc = graph.add_node("libc v0.2.4");
    let net2 = graph.add_node("net2 v0.2.20");
    let cfg_if = graph.add_node("cfg-if v0.1.0");
    let kernel32_sys = graph.add_node("kernel32-sys v0.2.1");
    let winapi = graph.add_node("winapi v0.2.5");
    let ws2_32_sys = graph.add_node("ws2_32-sys v0.2.1");
    let phf = graph.add_node("phf v0.7.9");
    let phf_shared = graph.add_node("phf_shared v0.7.9");
    let serde_json = graph.add_node("serde_json v0.6.0");
    let num = graph.add_node("num v0.1.29");
    let rand = graph.add_node("rand v0.3.12");
    let advapi32_sys = graph.add_node("advapi32-sys v0.1.2");
    let rustc_serialize = graph.add_node("rustc-serialize v0.3.16");
    let serde = graph.add_node("serde v0.6.7");
    let serde_num = graph.add_node("num v0.1.29");

    graph.extend_with_edges(&[(advapi32_sys, winapi),

                              (rand, winapi),
                              (rand, libc),
                              (rand, advapi32_sys),

                              (serde, serde_num),
                              (num, rustc_serialize),
                              (num, rand),
                              (ws2_32_sys, winapi),
                              (kernel32_sys, winapi),

                              (serde_json, serde),
                              (serde_json, num),
                              (phf, phf_shared),
                              (net2, ws2_32_sys),
                              (net2, winapi),
                              (net2, libc),
                              (net2, kernel32_sys),
                              (net2, cfg_if),
                              (log, libc),

                              (postgres, serde_json),
                              (postgres, phf),
                              (postgres, net2),
                              (postgres, log),
                              (postgres, hex),
                              (postgres, byteorder),
                              (postgres, bufstream)]);

    viztree::print(&graph, postgres, true);
}
