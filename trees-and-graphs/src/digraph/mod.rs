// This was harder than I expected to create. Normally I'd use an off-the-shelf library, but I
// wouldn't have learned anything from doing that, so...
// See: https://github.com/nrc/r4cppp/blob/master/graphs/README.md
//
// Its interface and implementation could be improved.
// It might be a good idea to read this article and try its recommendations.
// See: http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices

use std::rc::Rc;
use std::cell::RefCell;

type NodeIndex = usize;
type EdgeIndex = usize;

pub struct Node<NodeValue> {
    pub value: NodeValue,
    pub edges: Vec<Rc<RefCell<Edge<NodeValue>>>>
}

impl<NodeValue> Node<NodeValue> {
    pub fn new(value: NodeValue) -> Rc<RefCell<Node<NodeValue>>> {
        Rc::new(RefCell::new(Node {
            value: value,
            edges: Vec::new()
        }))
    }

    pub fn add_edge(&mut self, edge: Rc<RefCell<Edge<NodeValue>>>) {
        self.edges.push(edge)
    }
}

pub struct Edge<NodeValue> {
    pub weight: i32,
    pub source: Rc<RefCell<Node<NodeValue>>>,
    pub target: Rc<RefCell<Node<NodeValue>>>
}

impl<NodeValue> Edge<NodeValue> {
    pub fn new(source: Rc<RefCell<Node<NodeValue>>>, target: Rc<RefCell<Node<NodeValue>>>) -> Rc<RefCell<Edge<NodeValue>>> {
        Rc::new(RefCell::new(Edge {
            weight: 1,
            source: source,
            target: target
        }))
    }
}

pub struct Digraph<NodeValue> {
    pub nodes: Vec<Rc<RefCell<Node<NodeValue>>>>,
    pub edges: Vec<Rc<RefCell<Edge<NodeValue>>>>
}

impl<NodeValue> Digraph<NodeValue> {

    pub fn new() -> Digraph<NodeValue> {
        Digraph {
            nodes: Vec::new(),
            edges: Vec::new()
        }
    }

    pub fn add_node(&mut self, value: NodeValue) -> NodeIndex {
        let index = self.nodes.len();
        let node = Node::new(value);
        self.nodes.push(node);
        index
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) -> EdgeIndex {
        let index = self.edges.len();
        let source_node = self.nodes[source].clone();
        let target_node = self.nodes[target].clone();
        let edge = Edge::new(source_node, target_node);
        self.nodes[source].borrow_mut().add_edge(edge.clone());
        self.nodes[target].borrow_mut().add_edge(edge.clone());
        self.edges.push(edge);
        index
    }

}
