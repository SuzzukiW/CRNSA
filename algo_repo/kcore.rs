// kcore.rs

use petgraph::graph::Graph;
use petgraph::prelude::{EdgeRef, NodeIndex};
use petgraph::visit::NodeIndexable;
use petgraph::Undirected;

// Function that takes a graph and a value k as input, and returns a new graph
// containing only the nodes that are part of the k-core.
pub fn k_core_decomposition<N: Clone, E: Clone, Ty: petgraph::EdgeType>(
    graph: &Graph<N, E, Ty>,
    k: usize,
) -> Graph<N, E, Ty> {
    // Clone the input graph to work on a separate copy
    let mut core_graph = graph.clone();
    // Create a vector to store the degrees of each node in the graph
    let mut degrees = vec![0; graph.node_count()];

    // Initialize the degrees vector with the degrees of each node
    for node in graph.node_indices() {
        degrees[graph.to_index(node)] = graph.neighbors(node).count();
    }

    // Main loop to iteratively remove nodes with degree less than k
    loop {
        // Initialize a vector to store the nodes to be removed in the current iteration
        let mut nodes_to_remove = Vec::new();

        // Identify nodes with degree less than k
        for node in core_graph.node_indices() {
            if degrees[core_graph.to_index(node)] < k {
                nodes_to_remove.push(node);
            }
        }

        // If there are no nodes to remove, break the loop
        if nodes_to_remove.is_empty() {
            break;
        }

        // Remove the identified nodes and update the degrees of their neighbors
        for node in nodes_to_remove {
            for neighbor in core_graph.neighbors(node) {
                let neighbor_index = core_graph.to_index(neighbor);
                if degrees[neighbor_index] > 0 {
                    degrees[neighbor_index] -= 1;
                }
            }

            // Remove the node from the core_graph
            core_graph.remove_node(node);
        }
    }

    // Create a new graph to store the result of the k-core decomposition
    let mut result_graph = Graph::<N, E, Ty>::with_capacity(core_graph.node_count(), core_graph.edge_count());

    // Add the nodes from the core_graph to the result_graph
    for node in core_graph.node_indices() {
        let node_data = core_graph.node_weight(node).unwrap().clone();
        result_graph.add_node(node_data);
    }

    // Add the edges from the core_graph to the result_graph
    for edge in core_graph.edge_indices() {
        let edge_data = core_graph.edge_weight(edge).unwrap().clone();
        let (a, b) = core_graph.edge_endpoints(edge).unwrap();
        result_graph.add_edge(a, b, edge_data);
    }

    // Return the result_graph containing the k-core
    result_graph
}
