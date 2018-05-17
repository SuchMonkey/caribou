extern crate dotenv;
extern crate postgres;

use dotenv::dotenv;
use postgres::{Connection, TlsMode};
use std::env;

fn main() {
    let conn = establich_connection();
    let node_types = vec!["type_a".to_string(), "type_b".to_string()];

    create_table_node(&conn, &node_types);

    create_table_edge(&conn);

    for node_type in &node_types {
        create_table_type(&conn, &node_type);
    }
    
    
}

fn establich_connection() -> Connection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    Connection::connect(database_url, TlsMode::None).unwrap()
}

fn create_table_edge(conn: &Connection) {
    let query = "
    CREATE TABLE IF NOT EXISTS edge (
        from_node_id BIGINT NOT NULL REFERENCES node(id),
        to_node_id BIGINT NOT NULL REFERENCES node(id),
        CONSTRAINT edge_pkey PRIMARY KEY (from_node_id, to_node_id),
        CONSTRAINT not_self_referencial CHECK (from_node_id != to_node_id)
    )";

    println!("{:?}", query);

    conn.execute(query, &[]).unwrap();
}

fn create_table_node(conn: &Connection, node_types: &Vec<String>) {
    let node_types: Vec<String> = node_types.into_iter().map(|t| format!("'{}'", t)).collect();
    let node_types = node_types.join(",");

    // https://stackoverflow.com/questions/10068033/postgresql-foreign-key-referencing-primary-keys-of-two-different-tables
    let query = format!("    
    CREATE TABLE IF NOT EXISTS node (
        id BIGSERIAL PRIMARY KEY,
        node_type VARCHAR(50) NOT NULL,
        CONSTRAINT is_valid_type CHECK (node_type in ({})),
        CONSTRAINT is_unique_type UNIQUE (id, node_type)
    )", node_types);


    println!("{:?}", query);

    conn.execute(&query, &[]).unwrap();
}

fn create_table_type(conn: &Connection, node_type: &str) {
    let query = format!("
    CREATE TABLE IF NOT EXISTS {t} (
        id BIGSERIAL PRIMARY KEY REFERENCES node(id),
        node_type VARCHAR(50) NOT NULL DEFAULT '{t}',
        CONSTRAINT is_valid_type CHECK (node_type = '{t}'),
        FOREIGN KEY (id, node_type) REFERENCES node(id, node_type)
    )", t=node_type);

    println!("{:?}", query);

    conn.execute(&query, &[]).unwrap();
}
