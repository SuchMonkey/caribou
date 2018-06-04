mod node;

use node::{IntNode, Node, NodeState}

struct Network {
    redis_stuff: String,
}

impl Network {
    fn new() -> Self {
        Network {
            redis_stuff: "redis connection".to_string(),
        }
    }

    fn integrate(node: Node) -> IntNode {
        IntNode::new(node)
    }

    fn queu_action() {
        // add mutation action or other stuff to internal job q
        // basically translates to redis operations
        // redis is single threaded soo..
        // this would be the first location to look for when integrating
        // async io or multiple processes
        // mutation can only happen via this job q which defines callbacks
        // when a specific job gets scheduled the callback will be called
        // and the caller reciefes the current network state and can resolve
        // conflicts on spot
        // that's how it's done bby
        // this allows for all nodes to borrow a network but never does one need to have a mutation 
        // ref to borrow... now mutexes or mybe not?^^
        // network can alwasy cell mutatie as the only src of truth
    }
}
