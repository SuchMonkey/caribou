pub enum NodeState {
    INT_NODE,
    NODE,
}

pub struct IntNode<T> {
    node: T,
    net: &Network,
}

pub trait Node {
    fn new() -> self;
}
