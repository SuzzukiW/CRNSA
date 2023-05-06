// network_analysis.rs

use petgraph::graph::NodeIndex;
use petgraph::{Graph, Undirected};
use ndarray::{Array1, Axis};
use ndarray_stats::QuantileExt;

// Define a function called `degree_distribution` that takes a reference to a generic `Graph` with node properties of `(f64, f64)` type, edge weights of type `f64`, and undirected edges. The function computes the degree distribution of the nodes in the graph and returns it as a one-dimensional `Array1` object.
pub fn degree_distribution(graph: &Graph<(f64, f64), f64, Undirected>) -> Array1<usize> {
    // Compute the degree of each node and store it in a vector.
    let degrees: Vec<usize> = graph.node_indices().map(|n| graph.neighbors(n).count()).collect();
    // Compute the maximum degree in the graph.
    let max_degree = *degrees.iter().max().unwrap();
    // Initialize an array of zeros with a length of `max_degree + 1` to store the degree distribution.
    let mut distribution = Array1::zeros(max_degree + 1);

    // Iterate over all degrees and increment the corresponding element in the distribution array.
    for degree in degrees {
        distribution[degree] += 1;
    }

    // Return the degree distribution as an `Array1` object.
    distribution
}


// Define a function called `clustering_coefficient` that takes a reference to a generic `Graph` with node properties of `(f64, f64)` type, edge weights of type `f64`, and undirected edges. The function computes the clustering coefficient of the graph and returns it as a `f64` value.
pub fn clustering_coefficient(graph: &Graph<(f64, f64), f64, Undirected>) -> f64 {
    // Get the node indices of the graph.
    let nodes = graph.node_indices();
    // Initialize a variable to store the total clustering coefficient.
    let mut total_coefficient = 0.0;

    // Iterate over all nodes.
    for node in nodes {
        // Get the neighbors of the current node and store them in a vector.
        let neighbors: Vec<NodeIndex> = graph.neighbors(node).collect();
        // Get the degree of the current node.
        let k = neighbors.len();

        // If the degree is greater than 1...
        if k > 1 {
            // Initialize a variable to store the number of connected neighbors.
            let mut connected_neighbors = 0;

            // Iterate over all pairs of neighbors and count the number of pairs that are connected by an edge.
            for i in 0..k {
                for j in (i + 1)..k {
                    if graph.contains_edge(neighbors[i], neighbors[j]) {
                        connected_neighbors += 1;
                    }
                }
            }

            // Compute the clustering coefficient of the current node and add it to the total coefficient.
            let coefficient = 2.0 * (connected_neighbors as f64) / (k * (k - 1)) as f64;
            total_coefficient += coefficient;
        }
    }

    // Compute the average clustering coefficient of the graph and return it.
    total_coefficient / (graph.node_count() as f64)
}


// Define a function called `network_density` that takes a reference to a generic `Graph` with node properties of `(f64, f64)` type, edge weights of type `f64`, and undirected edges. The function computes the network density of the graph and returns it as a `f64` value.
pub fn network_density(graph: &Graph<(f64, f64), f64, Undirected>) -> f64 {
    // Get the number of nodes and edges in the graph and convert them to `f64` values.
    let node_count = graph.node_count() as f64;
    let edge_count = graph.edge_count() as f64;

    // Compute the network density of the graph using the formula: (2 * E) / (N * (N - 1)).
    (2.0 * edge_count) / (node_count * (node_count - 1.0))
}




#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_graph() -> Graph<(f64, f64), f64, Undirected> {
        let mut graph = Graph::<(f64, f64), f64, Undirected>::new_undirected();
        let a = graph.add_node((0.0, 0.0));
        let b = graph.add_node((1.0, 0.0));
        let c = graph.add_node((1.0, 1.0));

        graph.add_edge(a, b, 1.0);
        graph.add_edge(b, c, 1.0);

        graph
    }

    #[test]
    fn test_clustering_coefficient() {
        let graph = create_test_graph();
        let coefficient = clustering_coefficient(&graph);

        assert_eq!(coefficient, 0.0);
    }
}



