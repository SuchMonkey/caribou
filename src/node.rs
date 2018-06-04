use network::Network;

pub enum NodeState {
    IntNode,
    Node,
}

pub struct IntNode<'a, T> {
    node: T,
    net: &'a Network,
}

impl<'a, T> IntNode<'a, T> {
    pub fn new(net: &'a Network, node: T) -> Self {
        IntNode {
            node: node,
            net: net,
        }
    }
}

pub trait Node {
    fn new() -> Self;
}
