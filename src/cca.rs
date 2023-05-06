// cca.rs

use petgraph::Graph;
use petgraph::prelude::*;
use petgraph::algo::connected_components;
use std::collections::HashMap;

// The analyze_connected_components function takes a reference to an undirected graph with nodes
// representing points in 2D space and edges with associated weights. It returns a tuple containing
// the number of connected components in the graph and a vector of vectors, where each inner vector
// represents the nodes belonging to a specific connected component.
pub fn analyze_connected_components(graph: &Graph<(f64, f64), f64, petgraph::Undirected>) -> (usize, Vec<Vec<NodeIndex>>) {
    // Calculate the number of connected components in the graph using the connected_components function
    // from the petgraph crate.
    let num_connected_components = connected_components(&graph);

    // Initialize a HashMap to store the nodes belonging to each connected component, where the key is
    // the component ID and the value is a vector of NodeIndex values representing the nodes in the component.
    let mut components: HashMap<usize, Vec<NodeIndex>> = HashMap::new();

    // Iterate over all nodes in the graph and add them to the corresponding connected component in the HashMap.
    for node in graph.node_indices() {
        let component_id = graph[node].1 as usize;
        components.entry(component_id).or_insert_with(Vec::new).push(node);
    }

    // Convert the HashMap into a vector of vectors representing the connected components, and sort the
    // components by size in descending order.
    let mut component_sizes = components.into_iter().map(|(_id, nodes)| nodes).collect::<Vec<_>>();
    component_sizes.sort_by(|a, b| b.len().cmp(&a.len()));

    // Return the number of connected components and the sorted vector of connected components.
    (num_connected_components, component_sizes)
}



#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::NodeIndex;

    fn create_test_graph1() -> Graph<(f64, f64), f64, petgraph::Undirected> {
        let mut graph = Graph::<(f64, f64), f64, petgraph::Undirected>::new_undirected();
        let a = graph.add_node((0.0, 0.0));
        let b = graph.add_node((1.0, 0.0));
        graph.add_edge(a, b, 1.0);

        graph
    }

    fn create_test_graph2() -> Graph<(f64, f64), f64, petgraph::Undirected> {
        let mut graph = Graph::<(f64, f64), f64, petgraph::Undirected>::new_undirected();
        let a = graph.add_node((0.0, 0.0));
        let b = graph.add_node((1.0, 0.0));
        let c = graph.add_node((2.0, 1.0));
        graph.add_edge(a, b, 1.0);
        graph.add_edge(b, c, 1.0);

        graph
    }

    fn create_test_graph3() -> Graph<(f64, f64), f64, petgraph::Undirected> {
        let mut graph = Graph::<(f64, f64), f64, petgraph::Undirected>::new_undirected();
        let a = graph.add_node((0.0, 0.0));
        let b = graph.add_node((1.0, 0.0));
        let c = graph.add_node((2.0, 1.0));
        let d = graph.add_node((3.0, 1.0));
        graph.add_edge(a, b, 1.0);
        graph.add_edge(c, d, 1.0);

        graph
    }

    #[test]
    fn test_analyze_connected_components_single_component() {
        let graph = create_test_graph1();
        let (num_connected_components, component_sizes) = analyze_connected_components(&graph);

        assert_eq!(num_connected_components, 1);
        assert_eq!(component_sizes.len(), 1);
        assert_eq!(component_sizes[0].len(), 2);
    }

    #[test]
    fn test_analyze_connected_components_two_components() {
        let graph = create_test_graph3();
        let (num_connected_components, component_sizes) = analyze_connected_components(&graph);

        assert_eq!(num_connected_components, 2);
        assert_eq!(component_sizes.len(), 2);
        assert_eq!(component_sizes[0].len(), 2);
        assert_eq!(component_sizes[1].len(), 2);
    }

    #[test]
    fn test_analyze_connected_components_empty_graph() {
        let graph = Graph::<(f64, f64), f64, petgraph::Undirected>::new_undirected();
        let (num_connected_components, component_sizes) = analyze_connected_components(&graph);

        assert_eq!(num_connected_components, 0);
        assert_eq!(component_sizes.len(), 0);
    }
}

