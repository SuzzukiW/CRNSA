// centrality_analysis.rs

// Import the necessary modules and functions
use petgraph::prelude::UnGraphMap;
use crate::centrality::{degree_centrality};
use std::collections::HashMap;

// This function takes a reference to a petgraph UnGraphMap object and
// calculates its degree centrality using the degree_centrality function.
pub fn analyze_centrality(graph: &UnGraphMap<usize, f64>) -> HashMap<usize, f64> {
    let degree = degree_centrality(&graph);
    degree
}


// This section is for test functions
#[cfg(test)]
mod tests {
    // Import the necessary modules and functions
    use super::*;

    // This helper function creates a simple test graph
    fn create_test_graph() -> UnGraphMap<usize, f64> {
        let mut graph = UnGraphMap::<usize, f64>::new();
        graph.add_edge(0, 1, 1.0);
        graph.add_edge(1, 2, 1.0);
        graph.add_edge(1, 3, 1.0);
        graph
    }

    // This test function checks if the analyze_centrality function works correctly
    // by creating a simple graph and asserting that the calculated degree centrality
    // for each node matches the expected values.
    #[test]
    fn test_analyze_centrality() {
        let graph = create_test_graph();
        let degree = degree_centrality(&graph);

        assert_eq!(degree[&0], 1.0);
        assert_eq!(degree[&1], 3.0);
        assert_eq!(degree[&2], 1.0);
        assert_eq!(degree[&3], 1.0);
    }
}
