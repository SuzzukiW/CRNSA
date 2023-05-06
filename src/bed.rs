// bed.rs

use petgraph::graph::Graph;
use petgraph::Undirected;
use petgraph::visit::{Dfs, GraphRef, Visitable};
use petgraph::graph::NodeIndex;
use std::collections::HashSet;

// The find_bridge_edges function takes a reference to an undirected graph with nodes representing
// points in 2D space and edges with associated weights. It returns a HashSet containing pairs of
// NodeIndex values representing the bridge edges in the graph. A bridge edge is an edge whose removal
// increases the number of connected components in the graph.
pub fn find_bridge_edges(graph: &Graph<(f64, f64), f64, Undirected>) -> HashSet<(NodeIndex, NodeIndex)> {
    let mut bridges = HashSet::new();

    // If the graph is empty, return an empty set of bridges.
    if graph.node_count() == 0 {
        return bridges;
    }

    // Initialize a depth-first search (DFS) traversal starting from the first node in the graph.
    let mut dfs = Dfs::new(graph, NodeIndex::new(0));
    // Initialize vectors to store low-link values, discovery times, and parent nodes for each node.
    let mut low = vec![0; graph.node_count()];
    let mut disc = vec![0; graph.node_count()];
    let mut parent = vec![None; graph.node_count()];
    // Initialize a time counter for discovery times.
    let mut time = 0;

    // Perform the DFS traversal and update low-link values, discovery times, and parent nodes.
    while let Some(node) = dfs.next(graph) {
        if disc[node.index()] == 0 {
            bridge_dfs(graph, node, &mut low, &mut disc, &mut parent, &mut time, &mut bridges);
        }
    }

    // Return the set of bridge edges.
    bridges
}


fn bridge_dfs(
    graph: &Graph<(f64, f64), f64, Undirected>, // The input graph
    start: NodeIndex, // The starting node for the DFS traversal
    low: &mut Vec<usize>, // Vector to store low-link values for each node
    disc: &mut Vec<usize>, // Vector to store discovery times for each node
    parent: &mut Vec<Option<NodeIndex>>, // Vector to store parent nodes for each node
    time: &mut usize, // Counter for discovery times
    bridges: &mut HashSet<(NodeIndex, NodeIndex)>, // Set to store identified bridge edges
) {
    // Define an enum to represent the state of the DFS traversal (entering or exiting a node).
    enum State {
        Enter(NodeIndex),
        Exit(NodeIndex),
    }

    // Initialize a stack for the DFS traversal and push the starting node with the Enter state.
    let mut stack = vec![State::Enter(start)];

    // Perform the DFS traversal using the stack.
    while let Some(state) = stack.pop() {
        match state {
            State::Enter(u) => {
                // Increment the time counter and update the discovery time and low-link value for the current node.
                *time += 1;
                disc[u.index()] = *time;
                low[u.index()] = *time;

                // Push the current node with the Exit state onto the stack.
                stack.push(State::Exit(u));

                // Explore the neighbors of the current node.
                for neighbor in graph.neighbors(u) {
                    if disc[neighbor.index()] == 0 {
                        // If the neighbor is undiscovered, set its parent to the current node and push it with the Enter state.
                        parent[neighbor.index()] = Some(u);
                        stack.push(State::Enter(neighbor));
                    } else if parent[u.index()] != Some(neighbor) {
                        // If the neighbor is discovered and is not the parent of the current node, update the low-link value of the current node.
                        low[u.index()] = usize::min(low[u.index()], disc[neighbor.index()]);
                    }
                }
            }
            State::Exit(u) => {
                // When exiting a node, update the low-link value of its parent based on the low-link value of the current node.
                if let Some(parent_u) = parent[u.index()] {
                    low[parent_u.index()] = usize::min(low[parent_u.index()], low[u.index()]);
                    // If the low-link value of the current node is greater than the discovery time of its parent,
                    // then the edge between the current node and its parent is a bridge edge.
                    if low[u.index()] > disc[parent_u.index()] {
                        bridges.insert((parent_u, u));
                    }
                }
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::Graph;
    use petgraph::graph::NodeIndex;

    fn build_test_graph() -> Graph<(f64, f64), f64, Undirected> {
        let mut graph = Graph::<(f64, f64), f64, Undirected>::new_undirected();
        let nodes = (0..6).map(|_| graph.add_node((0.0, 0.0))).collect::<Vec<_>>();

        graph.add_edge(nodes[0], nodes[1], 1.0);
        graph.add_edge(nodes[1], nodes[2], 1.0);
        graph.add_edge(nodes[2], nodes[0], 1.0);
        graph.add_edge(nodes[1], nodes[3], 1.0);
        graph.add_edge(nodes[3], nodes[4], 1.0);
        graph.add_edge(nodes[4], nodes[5], 1.0);
        graph.add_edge(nodes[5], nodes[3], 1.0);

        graph
    }

    #[test]
    fn test_find_bridge_edges() {
        let graph = build_test_graph();
        let bridges = find_bridge_edges(&graph);
        let expected_bridges = vec![(NodeIndex::new(1), NodeIndex::new(3))];
        assert_eq!(bridges.len(), expected_bridges.len());
        for (u, v) in expected_bridges {
            assert!(bridges.contains(&(u, v)) || bridges.contains(&(v, u)));
        }
    }

    #[test]
    fn test_find_bridge_edges_empty_graph() {
        let graph = Graph::<(f64, f64), f64, Undirected>::new_undirected();
        let bridges = find_bridge_edges(&graph);
        assert!(bridges.is_empty());
    }

    #[test]
    fn test_find_bridge_edges_no_bridge_edges() {
        let mut graph = Graph::<(f64, f64), f64, Undirected>::new_undirected();
        let nodes = (0..3).map(|_| graph.add_node((0.0, 0.0))).collect::<Vec<_>>();

        graph.add_edge(nodes[0], nodes[1], 1.0);
        graph.add_edge(nodes[1], nodes[2], 1.0);
        graph.add_edge(nodes[2], nodes[0], 1.0);

        let bridges = find_bridge_edges(&graph);
        assert!(bridges.is_empty());
    }
}


