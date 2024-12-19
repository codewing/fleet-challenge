use std::{fs::File, io::BufReader};

use arculus_fleet_manager::domain::graph::Graph;

pub fn load_graph(file_name: &str) -> Result<Graph, std::io::Error> {
    let file = File::open(format!("./tests/data/{file_name}.json"))?;
    let reader = BufReader::new(file);
    let graph: Graph = serde_json::from_reader(reader)?;
    Ok(graph)
}
