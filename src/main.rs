extern crate redis;

mod network;
mod node;

use node::Node;

struct Asdf {
    dummy: String,
}

impl Node for Asdf {
    fn new() -> Self {
        Asdf {
            dummy: "hi".to_string(),
        }
    }
}

fn main() {
    let mut net = network::Network::new();
    let node = Asdf::new();
    let node = net.integrate(node);
}
