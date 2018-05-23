extern crate postgres;

use self::postgres::{Connection, TlsMode};
use node_factory::NodeFactory;

pub struct Graph {
    connection: Connection,
    node_factory_register: Vec<Box<NodeFactory>>,
}

impl Graph {
    pub fn new(database_url: String) -> Self {
        Graph {
            connection: Connection::connect(database_url, TlsMode::None).unwrap(),
            node_factory_register: Vec::new(),
        }
    }

    pub fn register_node_factory(&mut self, node_factory: Box<NodeFactory>) {
        self.node_factory_register.push(node_factory);
    }

    pub fn initialize_db(&self) {
        let node_types: Vec<String> = self.node_factory_register
            .iter()
            .map(|t| format!("'{}'", t.name()))
            .collect();

        self.create_table_node(&node_types.join(","));
        self.create_table_edge();

        for node_type in &node_types {
            self.create_table_type(&node_type);
        }
    }

    fn create_table_edge(&self) {
        let query = "
            CREATE TABLE IF NOT EXISTS edge (
                from_node_id BIGINT NOT NULL REFERENCES node(id),
                to_node_id BIGINT NOT NULL REFERENCES node(id),
                CONSTRAINT edge_pkey PRIMARY KEY (from_node_id, to_node_id),
                CONSTRAINT not_self_referencial CHECK (from_node_id != to_node_id)
            )";

        println!("{:?}", query);

        self.connection.execute(query, &[]).unwrap();
    }

    fn create_table_node(&self, node_types: &str) {
        // https://stackoverflow.com/questions/10068033/postgresql-foreign-key-referencing-primary-keys-of-two-different-tables
        let query = format!(
            "
            CREATE TABLE IF NOT EXISTS node (
                id BIGSERIAL PRIMARY KEY,
                node_type VARCHAR(50) NOT NULL,
                CONSTRAINT is_valid_type CHECK (node_type in ({})),
                CONSTRAINT is_unique_node UNIQUE (id, node_type)
            )",
            node_types
        );

        println!("{:?}", query);

        self.connection.execute(&query, &[]).unwrap();
    }

    fn create_table_type(&self, node_type: &str) {
        let query = format!(
            "
            CREATE TABLE IF NOT EXISTS node_{t} (
                id BIGSERIAL PRIMARY KEY REFERENCES node(id),
                node_type VARCHAR(50) NOT NULL DEFAULT '{t}',
                CONSTRAINT is_valid_type CHECK (node_type = '{t}'),
                FOREIGN KEY (id, node_type) REFERENCES node(id, node_type)
            )",
            t = node_type
        );

        println!("{:?}", query);

        self.connection.execute(&query, &[]).unwrap();
    }
}
