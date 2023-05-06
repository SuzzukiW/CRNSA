// assort.rs

use petgraph::visit::{EdgeRef, IntoNeighbors, NodeIndexable};
use petgraph::Graph;
use std::collections::HashMap;

pub fn calculate_assortativity_coefficient<N, E, Ty: petgraph::EdgeType>(
    graph: &Graph<N, E, Ty>,
) -> f64 {
    let mut sum1 = 0.0; // Sum of the product of degrees of connected nodes
    let mut sum2 = 0.0; // Sum of the degrees of all nodes
    let mut sum3 = 0.0; // Sum of the square of degrees of all nodes
    let mut edge_count = 0.0; // Total number of edges in the graph

    // Iterate over all edges in the graph
    for edge in graph.edge_references() {
        let source_degree = graph.neighbors(edge.source()).count(); // Degree of the source node
        let target_degree = graph.neighbors(edge.target()).count(); // Degree of the target node

        sum1 += (source_degree * target_degree) as f64;
        sum2 += (source_degree + target_degree) as f64;
        sum3 += (source_degree * source_degree + target_degree * target_degree) as f64;

        edge_count += 1.0;
    }

    // Calculate the numerator and denominator for the assortativity coefficient formula
    let numerator = edge_count * sum1 - (sum2 * 0.5).powi(2);
    let denominator = edge_count * sum3 - (sum2 * 0.5).powi(2);

    // Calculate the assortativity coefficient
    // If the denominator is not zero, return the calculated value, otherwise return 0.0
    if denominator != 0.0 {
        numerator / denominator
    } else {
        0.0
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::Undirected;

    fn create_linear_graph() -> Graph<(), (), Undirected> {
        let mut graph = Graph::new_undirected();
        let n1 = graph.add_node(());
        let n2 = graph.add_node(());
        let n3 = graph.add_node(());
        let n4 = graph.add_node(());

        graph.add_edge(n1, n2, ());
        graph.add_edge(n2, n3, ());
        graph.add_edge(n3, n4, ());

        graph
    }

    fn create_star_graph() -> Graph<(), (), Undirected> {
        let mut graph = Graph::new_undirected();
        let center = graph.add_node(());
        let n1 = graph.add_node(());
        let n2 = graph.add_node(());
        let n3 = graph.add_node(());

        graph.add_edge(center, n1, ());
        graph.add_edge(center, n2, ());
        graph.add_edge(center, n3, ());

        graph
    }

    #[test]
    fn test_calculate_assortativity_coefficient_empty_graph() {
        let graph = Graph::<(), (), Undirected>::new_undirected();
        let result = calculate_assortativity_coefficient(&graph);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_calculate_assortativity_coefficient_single_node() {
        let mut graph = Graph::<(), (), Undirected>::new_undirected();
        graph.add_node(());
        let result = calculate_assortativity_coefficient(&graph);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_calculate_assortativity_coefficient_linear_graph() {
        let graph = create_linear_graph();
        let result = calculate_assortativity_coefficient(&graph);
        assert!((result - (-0.034482758620689655)).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_assortativity_coefficient_star_graph() {
        let graph = create_star_graph();
        let result = calculate_assortativity_coefficient(&graph);
        assert!((result - (-0.16666666666666666)).abs() < 1e-10);
    }

}


