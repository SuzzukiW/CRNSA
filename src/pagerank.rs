// pagerank.rs

use petgraph::graph::NodeIndex;
use petgraph::Graph;
use petgraph::Undirected;
use std::iter::repeat_with;

// Take a reference to a generic `Graph` with node properties of `(f64, f64)` type, edge weights of type `f64`, and undirected edges; a damping factor `f64`; and the number of iterations to run the PageRank algorithm as a `usize` value. The function computes the PageRank scores of the nodes in the graph and returns them as a vector of `(NodeIndex, f64)` tuples.
pub fn pagerank(
    graph: &Graph<(f64, f64), f64, Undirected>,
    damping_factor: f64,
    iterations: usize,
) -> Vec<(NodeIndex, f64)> {
    // Compute the number of nodes in the graph and the initial value for each node.
    let node_count = graph.node_count() as f64;
    let initial_value = 1.0 / node_count;

    // Initialize a vector to store the PageRank scores for each node.
    let mut ranks: Vec<f64> = repeat_with(|| initial_value).take(graph.node_count()).collect();
    let mut new_ranks: Vec<f64> = vec![0.0; graph.node_count()];

    // Compute the sum of the damping factor for use in the PageRank algorithm.
    let damping_factor_sum = (1.0 - damping_factor) / node_count;

    // Iterate over the specified number of iterations for the PageRank algorithm.
    for _ in 0..iterations {
        // Compute the sum of the PageRank scores for dangling nodes.
        let dangling_nodes = graph.node_indices().filter(|&node| graph.neighbors(node).count() == 0);
        let dangling_sum: f64 = dangling_nodes.map(|node| ranks[node.index()]).sum();

        // Iterate over all nodes in the graph.
        for node in graph.node_indices() {
            // Compute the sum of the PageRank scores for neighbors of the current node.
            let sum: f64 = graph
                .neighbors(node)
                .map(|neighbor| {
                    let degree = graph.neighbors(neighbor).count();
                    if degree > 0 {
                        ranks[neighbor.index()] / degree as f64
                    } else {
                        0.0
                    }
                })
                .sum();

            // Compute the new PageRank score for the current node and store it in the `new_ranks` vector.
            new_ranks[node.index()] =
                damping_factor_sum + damping_factor * (sum + dangling_sum / node_count);
        }

        // Update the `ranks` vector with the new PageRank scores.
        ranks.clone_from_slice(&new_ranks);
    }

    // Sort the nodes by their PageRank scores and return the result as a vector of `(NodeIndex, f64)` tuples.
    let mut result: Vec<(NodeIndex, f64)> = graph
        .node_indices()
        .map(|n| (n, ranks[n.index()]))
        .collect();
    result.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    result
}



#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::NodeIndex;
    use petgraph::{Graph, Undirected};

    #[test]
    fn test_pagerank() {
        let mut graph = Graph::<(f64, f64), f64, Undirected>::new_undirected();
        let a = graph.add_node((0.0, 0.0));
        let b = graph.add_node((0.0, 0.0));
        let c = graph.add_node((0.0, 0.0));
        let d = graph.add_node((0.0, 0.0));

        graph.extend_with_edges(&[
            (a, b, 1.0),
            (b, c, 1.0),
            (c, d, 1.0),
            (d, a, 1.0),
            (a, c, 1.0),
            (b, d, 1.0),
        ]);

        let damping_factor = 0.85;
        let iterations = 100;
        let result = pagerank(&graph, damping_factor, iterations);

        assert_eq!(result.len(), 4);
        assert!(result.iter().all(|(_, rank)| *rank > 0.0));

        let expected_result = vec![
            (NodeIndex::new(0), 0.25),
            (NodeIndex::new(1), 0.25),
            (NodeIndex::new(2), 0.25),
            (NodeIndex::new(3), 0.25),
        ];

        for (i, (node, rank)) in result.iter().enumerate() {
            assert_eq!(*node, expected_result[i].0);
            assert!((rank - expected_result[i].1).abs() < 1e-6);
        }
    }

    #[test]
    fn test_pagerank_single_node() {
        let mut graph = Graph::<(f64, f64), f64, Undirected>::new_undirected();
        let a = graph.add_node((0.0, 0.0));

        let damping_factor = 0.85;
        let iterations = 100;
        let result = pagerank(&graph, damping_factor, iterations);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, a);
        assert!((result[0].1 - 1.0).abs() < 1e-4);
    }

    #[test]
    fn test_pagerank_disconnected_graph() {
        let mut graph = Graph::<(f64, f64), f64, Undirected>::new_undirected();
        let a = graph.add_node((0.0, 0.0));
        let b = graph.add_node((0.0, 0.0));
        let c = graph.add_node((0.0, 0.0));
        let d = graph.add_node((0.0, 0.0));

        let damping_factor = 0.85;
        let iterations = 100;
        let result = pagerank(&graph, damping_factor, iterations);

        assert_eq!(result.len(), 4);
        assert!(result.iter().all(|(_, rank)| *rank > 0.0));

        let expected_value = 1.0 / 4.0;
        for (_, rank) in result {
            assert!((rank - expected_value).abs() < 1e-4);
        }
    }

    #[test]
    fn test_pagerank_directed_cycle() {
        let mut graph = Graph::<(f64, f64), f64, Undirected>::new_undirected();
        let a = graph.add_node((0.0, 0.0));
        let b = graph.add_node((0.0, 0.0));
        let c = graph.add_node((0.0, 0.0));

        graph.extend_with_edges(&[
            (a, b, 1.0),
            (b, c, 1.0),
            (c, a, 1.0),
        ]);

        let damping_factor = 0.85;
        let iterations = 100;
        let result = pagerank(&graph, damping_factor, iterations);

        assert_eq!(result.len(), 3);
        assert!(result.iter().all(|(_, rank)| *rank > 0.0));

        let expected_value = 1.0 / 3.0;
        for (_, rank) in result {
            assert!((rank - expected_value).abs() < 1e-4);
        }
    }

    #[test]
    fn test_pagerank_star_graph() {
        let mut graph = Graph::<(f64, f64), f64, Undirected>::new_undirected();
        let center = graph.add_node((0.0, 0.0));
        let mut nodes = Vec::new();

        for _ in 0..4 {
            let node = graph.add_node((0.0, 0.0));
            graph.add_edge(center, node, 1.0);
            nodes.push(node);
        }

        let damping_factor = 0.85;
        let iterations = 100;
        let result = pagerank(&graph, damping_factor, iterations);

        assert_eq!(result.len(), 5);
        assert!(result.iter().all(|(_, rank)| *rank > 0.0));

        let center_rank = result.iter().find(|(n, _)| *n == center).unwrap().1;
        for node in nodes {
            let node_rank = result.iter().find(|(n, _)| *n == node).unwrap().1;
            assert!(center_rank > node_rank);
        }
    }
}






