mod digraph;

use digraph::Digraph;

/*
 * Given a directed graph, design an algorithm to find out whether there is a route between two
 * nodes.
 */

// Implement DFS for two separate nodes within a graph and finish if/when they connect.

// Use:
// collections::vec_deque::VecDeque

fn main() {
    let mut graph = Digraph::new();
    let a = graph.add_node("hello".to_string());
    let b = graph.add_node("there".to_string());
    let c = graph.add_node("mate".to_string());

    graph.add_edge(a, b);
    graph.add_edge(a, c);

    println!("example value: {}", graph.nodes[0].borrow().value);
    println!("example weight: {}", graph.edges[0].borrow().weight);

    println!("total nodes: {}", graph.nodes.len());
    println!("total edges: {}", graph.edges.len());

    println!("edges of first node: {}", graph.nodes[0].borrow().edges.len());

    let ref source_node = graph.edges[0].borrow().source;
    let ref target_node = graph.edges[0].borrow().target;
    println!("source node of first edge: {}", source_node.borrow().value);
    println!("target node of first edge: {}", target_node.borrow().value);
}
