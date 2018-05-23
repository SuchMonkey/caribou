extern crate dotenv;

mod graph;
mod node_factory;

use dotenv::dotenv;
use graph::Graph;
use node_factory::NodeFactory;
use std::env;

struct Asdf {}
struct Bsdf {}

impl NodeFactory for Asdf {
    fn name(&self) -> String {
        "asdf".to_string()
    }
}

impl NodeFactory for Bsdf {
    fn name(&self) -> String {
        "bsdf".to_string()
    }
}

fn main() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut graph: Graph = Graph::new(database_url);

    let asdf = Asdf {};
    let bsdf = Bsdf {};

    graph.register_node_factory(Box::new(asdf));
    graph.register_node_factory(Box::new(bsdf));
    graph.initialize_db();
}
