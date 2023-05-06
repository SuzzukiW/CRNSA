// data.rs

use petgraph::graphmap::UnGraphMap;
use petgraph::algo::{connected_components, kosaraju_scc};
use petgraph::visit::{IntoEdgeReferences, NodeCount, EdgeCount};
use std::io::{BufRead, BufReader};
use std::fs::File;
use flate2::read::GzDecoder;

// Define a type alias for the road network graph
pub type RoadNetwork = UnGraphMap<usize, f64>;

// Function to read the input data from the given file and preprocess it into a graph
pub fn read_and_preprocess_data(file_path: &str) -> RoadNetwork {
    let file = File::open(file_path).expect("Failed to open file");

    // Check the file extension and create the appropriate reader
    let reader: Box<dyn BufRead> = if file_path.ends_with(".gz") {
        Box::new(BufReader::new(GzDecoder::new(file)))
    } else {
        Box::new(BufReader::new(file))
    };

    let mut graph = RoadNetwork::new();

    // Read each line from the input file and process it
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        // Ignore lines starting with '#'
        if line.starts_with('#') {
            continue;
        }

        // Parse the line into nodes
        let nodes: Vec<usize> = line
            .split_whitespace()
            .map(|n| n.parse::<usize>().expect("Failed to parse node"))
            .collect();

        let node_a = nodes[0];
        let node_b = nodes[1];

        // Add an edge between the nodes with a weight of 1.0
        graph.add_edge(node_a, node_b, 1.0);
    }

    graph
}

// Function to calculate and print the number of weakly and strongly connected components in the graph
pub fn network_properties(graph: &RoadNetwork) {
    // Weakly Connected Components
    let wcc = connected_components(graph);
    println!("Number of weakly connected components: {}", wcc);

    // Strongly Connected Components
    let scc = kosaraju_scc(graph);
    println!("Number of strongly connected components: {}", scc.len());
}


#[test]
fn test_empty_graph() {
    let graph: RoadNetwork = UnGraphMap::new();
    network_properties(&graph);
}


#[test]
fn test_single_edge_graph() {
    let mut graph: RoadNetwork = UnGraphMap::new();
    graph.add_edge(1, 2, 1.0);
    network_properties(&graph);
}


#[test]
fn test_disconnected_graph() {
    let mut graph: RoadNetwork = UnGraphMap::new();
    graph.add_edge(1, 2, 1.0);
    graph.add_edge(3, 4, 1.0);
    network_properties(&graph);
}


#[test]
fn test_connected_graph() {
    let mut graph: RoadNetwork = UnGraphMap::new();
    graph.add_edge(1, 2, 1.0);
    graph.add_edge(2, 3, 1.0);
    graph.add_edge(3, 4, 1.0);
    network_properties(&graph);
}

#[test]
fn test_self_loop_graph() {
    let mut graph: RoadNetwork = UnGraphMap::new();
    graph.add_edge(1, 1, 1.0);
    network_properties(&graph);
}


#[test]
fn test_multiple_edge_graph() {
    let mut graph: RoadNetwork = UnGraphMap::new();
    graph.add_edge(1, 2, 1.0);
    graph.add_edge(2, 3, 1.0);
    graph.add_edge(3, 4, 1.0);
    graph.add_edge(4, 5, 1.0);
    graph.add_edge(5, 1, 1.0);
    network_properties(&graph);
}



#[test]
fn test_single_node_graph() {
    let mut graph: RoadNetwork = UnGraphMap::new();
    graph.add_node(1);
    network_properties(&graph);
}


#[test]
fn test_disconnected_nodes_graph() {
    let mut graph: RoadNetwork = UnGraphMap::new();
    graph.add_node(1);
    graph.add_node(2);
    graph.add_node(3);
    network_properties(&graph);
}


#[test]
fn test_bipartite_graph() {
    let mut graph: RoadNetwork = UnGraphMap::new();
    graph.add_edge(1, 4, 1.0);
    graph.add_edge(1, 5, 1.0);
    graph.add_edge(2, 4, 1.0);
    graph.add_edge(2, 5, 1.0);
    graph.add_edge(3, 4, 1.0);
    graph.add_edge(3, 5, 1.0);
    network_properties(&graph);
}