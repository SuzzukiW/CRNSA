// LPA.rs

use petgraph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use std::collections::HashMap;

pub fn label_propagation(graph: &Graph<(f64, f64), f64, petgraph::Undirected>) -> Vec<Vec<NodeIndex>> {
    let mut labels: HashMap<NodeIndex, usize> = graph
        .node_indices()
        .enumerate()
        .map(|(index, node)| (node, index))
        .collect();

    let mut changed = true;
    while changed {
        changed = false;
        let nodes: Vec<NodeIndex> = graph.node_indices().collect();
        for node in nodes {
            let mut neighbor_labels: HashMap<usize, usize> = HashMap::new();
            for edge in graph.edges(node) {
                let label = labels[&edge.target()];
                *neighbor_labels.entry(label).or_insert(0) += 1;
            }

            let (new_label, _) = neighbor_labels
                .into_iter()
                .max_by_key(|(_, count)| *count)
                .unwrap_or((labels[&node], 0));

            if new_label != labels[&node] {
                labels.insert(node, new_label);
                changed = true;
            }
        }
    }

    let mut communities: HashMap<usize, Vec<NodeIndex>> = HashMap::new();
    for (node, label) in labels {
        communities.entry(label).or_insert_with(Vec::new).push(node);
    }

    communities.into_iter().map(|(_, community)| community).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::UnGraph;
    use petgraph::prelude::*;

    fn create_test_graph() -> UnGraph<(f64, f64), f64> {
        let mut graph = UnGraph::<_, f64>::new_undirected();

        let nodes: Vec<_> = (0..6).map(|_| graph.add_node((0.0, 0.0))).collect();

        graph.add_edge(nodes[0], nodes[1], 1.0);
        graph.add_edge(nodes[0], nodes[2], 1.0);
        graph.add_edge(nodes[1], nodes[2], 1.0);
        graph.add_edge(nodes[1], nodes[3], 1.0);
        graph.add_edge(nodes[2], nodes[3], 1.0);
        graph.add_edge(nodes[3], nodes[4], 1.0);
        graph.add_edge(nodes[3], nodes[5], 1.0);
        graph.add_edge(nodes[4], nodes[5], 1.0);

        graph
    }

    #[test]
    fn test_label_propagation() {
        let graph = create_test_graph();
        let communities = label_propagation(&graph);

        let expected_communities: Vec<Vec<NodeIndex>> = vec![
            vec![NodeIndex::new(0), NodeIndex::new(1), NodeIndex::new(2)],
            vec![NodeIndex::new(3), NodeIndex::new(4), NodeIndex::new(5)],
        ];

        assert_eq!(communities.len(), expected_communities.len());

        for community in communities {
            assert!(expected_communities.iter().any(|c| c == &community));
        }
    }
}
