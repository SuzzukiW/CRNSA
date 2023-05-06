// Core-Periphery Analysis

use petgraph::Graph;
use petgraph::Undirected;
use petgraph::graph::NodeIndex;
use std::collections::HashSet;

pub fn core_periphery_analysis(
    graph: &Graph<(f64, f64), f64, Undirected>,
    degree_threshold: usize,
) -> (HashSet<NodeIndex>, HashSet<NodeIndex>) {
    let mut core_nodes = HashSet::new(); // Set to store the core nodes
    let mut periphery_nodes = HashSet::new(); // Set to store the periphery nodes

    // Iterate over all nodes in the graph
    for node in graph.node_indices() {
        let degree = graph.neighbors(node).count(); // Calculate the degree of the node
        // If the degree of the node is greater than or equal to the specified threshold,
        // classify the node as a core node, otherwise classify it as a periphery node.
        if degree >= degree_threshold {
            core_nodes.insert(node);
        } else {
            periphery_nodes.insert(node);
        }
    }

    // Return the sets of core and periphery nodes
    (core_nodes, periphery_nodes)
}


#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::Graph;

    #[test]
    fn test_core_periphery_analysis() {
        let mut graph = Graph::<(f64, f64), f64, Undirected>::new_undirected();
        let a = graph.add_node((0.0, 0.0));
        let b = graph.add_node((1.0, 1.0));
        let c = graph.add_node((2.0, 2.0));
        let d = graph.add_node((3.0, 3.0));
        let e = graph.add_node((4.0, 4.0));

        graph.add_edge(a, b, 1.0);
        graph.add_edge(b, c, 1.0);
        graph.add_edge(c, d, 1.0);
        graph.add_edge(d, e, 1.0);

        let degree_threshold = 2;
        let (core_nodes, periphery_nodes) = core_periphery_analysis(&graph, degree_threshold);

        assert_eq!(core_nodes, vec![b, c, d].into_iter().collect());
        assert_eq!(periphery_nodes, vec![a, e].into_iter().collect());
    }
}
