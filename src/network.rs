use redis;
use redis::{Connection, RedisResult};

use node::{IntNode, Node, NodeState};

pub struct Network {
    client: RedisResult<Connection>,
}

impl Network {
    pub fn new(url: &str) -> Self {
        let client = try!(redis::Client::open(url));

        Network {
            client: try!(client.get_connection()),
        }
    }

    pub fn integrate<'a, T>(&'a self, node: T) -> IntNode<'a, T> {
        IntNode::new(&self, node)
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
