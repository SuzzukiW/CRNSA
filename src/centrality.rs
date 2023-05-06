// centrality.rs

use petgraph::graphmap::UnGraphMap;
use petgraph::visit::{EdgeRef, IntoEdgeReferences, NodeCount, EdgeCount};
use ndarray::{Array1, Array2, s};
use ndarray_linalg::c64;
use lapack::dsyev;
use std::collections::{HashMap, VecDeque};
use rayon::prelude::*;

// This function calculates the degree centrality of a given graph.
// It takes a reference to a petgraph UnGraphMap object as input and
// returns a HashMap where the keys are the node indices and the values
// are the degree centrality of each node.

pub fn degree_centrality(graph: &UnGraphMap<usize, f64>) -> HashMap<usize, f64> {
    // Create an empty HashMap to store the degree centrality values
    let mut centrality = HashMap::new();

    // Iterate over all nodes in the graph
    for node in graph.nodes() {
        // Count the number of neighbors for the current node
        let degree = graph.neighbors(node).count();

        // Insert the node index and its degree centrality into the HashMap
        centrality.insert(node, degree as f64);
    }

    // Return the degree centrality HashMap
    centrality
}

use std::time::Instant;
use rand::Rng;

// This test function checks if the degree_centrality function works correctly
// by creating a simple graph and asserting that the calculated degree centrality
// for each node matches the expected values.

#[test]
fn test_degree_centrality() {
    // Create a simple graph with 4 nodes and 4 edges
    let mut graph = UnGraphMap::new();
    graph.add_edge(0, 1, 1.0);
    graph.add_edge(0, 2, 1.0);
    graph.add_edge(1, 2, 1.0);
    graph.add_edge(1, 3, 1.0);

    // Calculate the degree centrality of the graph
    let centrality = degree_centrality(&graph);

    // Check if the calculated degree centrality values match the expected values
    assert_eq!(centrality[&0], 2.0);
    assert_eq!(centrality[&1], 3.0);
    assert_eq!(centrality[&2], 2.0);
    assert_eq!(centrality[&3], 1.0);
}
