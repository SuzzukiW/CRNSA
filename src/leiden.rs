// leiden.rs

use petgraph::graph::Graph;
use petgraph::visit::{EdgeRef, IntoNodeIdentifiers};
use petgraph::Undirected;
use std::collections::HashMap;
use crate::NodeIndex;

// Define a public function called `leiden_communities` that takes a reference to a generic `Graph` with node properties of `(f64, f64)` type, edge weights of type `f64`, and undirected edges, and returns a `HashMap` that maps each node to its community assignment.
pub fn leiden_communities(
    graph: &Graph<(f64, f64), f64, Undirected>,
) -> HashMap<usize, usize> {
    // Create a mutable copy of the input graph.
    let mut graph = graph.clone();
    // Initialize the community assignments using the `initial_community_assignments` function. The `&graph` argument is a reference to the input graph.
    let mut community_assignments = initial_community_assignments(&graph);
    // Initialize the iteration counter to zero.
    let mut iterations = 0;

    // Start a while loop that runs until either convergence is achieved or the maximum number of iterations is reached. In this case, the loop will run for 10 iterations.
    while iterations < 10 {
        // Call the `local_moving` function, passing in references to the input graph and the community assignments.
        local_moving(&graph, &mut community_assignments);

        // Call the `refinement` function, passing in references to the input graph and the current community assignments. The function returns a tuple containing the refined graph, the refined community assignments, and a Boolean flag indicating whether convergence was achieved. The underscore (_) is used to ignore the flag, since it is not needed here.
        let (new_graph, new_community_assignments, _) = refinement(&graph, &community_assignments);

        // Update the graph and community assignments with the refined versions obtained from the `refinement` function.
        graph = new_graph;
        community_assignments = new_community_assignments;

        // Increment the iteration counter at the end of each loop iteration.
        iterations += 1;
    }

    // Return the community assignments as a `HashMap`.
    community_assignments
}


// Define a function called `initial_community_assignments` that takes a reference to a generic `Graph` with node properties of `(f64, f64)` type, edge weights of type `f64`, and undirected edges, and returns a `HashMap` that maps each node to its initial community assignment.
fn initial_community_assignments(
    graph: &Graph<(f64, f64), f64, Undirected>,
) -> HashMap<usize, usize> {
    // Create a new, empty `HashMap` to store the community assignments.
    let mut community_assignments = HashMap::new();

    // Iterate over all the nodes in the input graph.
    for node in graph.node_indices() {
        // Get the index of the current node.
        let node_id = node.index();
        // Insert a new key-value pair into the `community_assignments` map, where the node's ID maps to its own index. This initializes each node to its own community.
        community_assignments.insert(node_id, node_id);
    }

    // Return the `community_assignments` map.
    community_assignments
}


// Define a function called `local_moving` that takes a reference to a generic `Graph` with node properties of `(f64, f64)` type, edge weights of type `f64`, and undirected edges, and a mutable reference to a `HashMap` that maps each node to its community assignment. The function performs local moving, which is a step of the Leiden algorithm.
fn local_moving(graph: &Graph<(f64, f64), f64, Undirected>, community_assignments: &mut HashMap<usize, usize>) {
    // Initialize a flag that indicates whether any improvement has been made.
    let mut improvement = true;

    // Start a while loop that runs until no improvement has been made.
    while improvement {
        // Set the improvement flag to false at the start of each iteration.
        improvement = false;

        // Iterate over all the nodes in the input graph.
        for node in graph.node_indices() {
            // Get the community assignment of the current node.
            let node_community = community_assignments[&node.index()];
            // Initialize variables to store the best community and the corresponding increase in modularity.
            let mut best_community = node_community;
            let mut best_delta_modularity = 0.0;

            // Iterate over all the neighbors of the current node.
            for neighbor in graph.neighbors(node) {
                // Get the community assignment of the current neighbor.
                let neighbor_community = community_assignments[&neighbor.index()];

                // If the neighbor belongs to a different community than the current node...
                if neighbor_community != node_community {
                    // Compute the increase in modularity that would result from moving the current node to the neighbor's community.
                    let delta_modularity = modularity_delta(
                        &graph,
                        &community_assignments,
                        node.index(),
                        node_community,
                        neighbor_community,
                    );

                    // If the increase in modularity is the best seen so far, update the best community and the corresponding increase in modularity.
                    if delta_modularity > best_delta_modularity {
                        best_delta_modularity = delta_modularity;
                        best_community = neighbor_community;
                    }
                }
            }

            // If the best community is different from the current community of the node, move the node to the best community and set the improvement flag to true.
            if best_community != node_community {
                community_assignments.insert(node.index(), best_community);
                improvement = true;
            }
        }
    }
}


// Define a function called `modularity_delta` that takes a reference to a generic `Graph` with node properties of `(f64, f64)` type, edge weights of type `f64`, and undirected edges, a `HashMap` that maps each node to its community assignment, the index of the node to be moved, the index of the community from which the node will be moved, and the index of the community to which the node will be moved. The function computes the increase in modularity that would result from moving the node from its current community to the new community.
fn modularity_delta(
    graph: &Graph<(f64, f64), f64, Undirected>,
    community_assignments: &HashMap<usize, usize>,
    node: usize,
    from_community: usize,
    to_community: usize,
) -> f64 {
    // Convert the node index to a `NodeIndex` object.
    let node_index = NodeIndex::new(node);
    // Initialize variables to store the total edge weight between the node and the communities, as well as the total weight of the node's edges.
    let mut from_community_weight = 0.0;
    let mut to_community_weight = 0.0;
    let mut node_weight = 0.0;

    // Iterate over all the edges incident to the node.
    for edge in graph.edges(node_index) {
        // Get the community assignment of the neighbor node.
        let neighbor_community = community_assignments[&edge.target().index()];
        // Get the weight of the current edge.
        let edge_weight = *edge.weight();
        // Add the edge weight to the total node weight.
        node_weight += edge_weight;

        // If the neighbor belongs to the community from which the node will be moved, add the edge weight to the total weight between the node and the community.
        if neighbor_community == from_community {
            from_community_weight += edge_weight;
        }
        // If the neighbor belongs to the community to which the node will be moved, add the edge weight to the total weight between the node and the community.
        else if neighbor_community == to_community {
            to_community_weight += edge_weight;
        }
    }

    // Compute the increase in modularity that would result from moving the node to the new community.
    let delta_modularity = to_community_weight / node_weight - from_community_weight / node_weight;

    // Return the delta modularity value.
    delta_modularity
}



// Define a function called `refinement` that takes a reference to a generic `Graph` with node properties of `(f64, f64)` type, edge weights of type `f64`, and undirected edges, and a `HashMap` that maps each node to its community assignment. The function performs refinement, which is a step of the Leiden algorithm that aggregates nodes that belong to the same community and constructs a new graph where the communities are nodes.
fn refinement(
    graph: &Graph<(f64, f64), f64, Undirected>,
    community_assignments: &HashMap<usize, usize>,
) -> (Graph<(f64, f64), f64, Undirected>, HashMap<usize, usize>, HashMap<usize, usize>) {
    // Create a new empty graph to store the communities as nodes.
    let mut new_graph = Graph::<(f64, f64), f64, Undirected>::new_undirected();
    // Create a new empty hash map to store the mapping between original communities and new nodes.
    let mut mapping = HashMap::new();

    // Iterate over all nodes and their community assignments.
    for (node, community) in community_assignments.iter() {
        // If the community has not already been added to the new graph, add it and create a mapping between the community and the new node index.
        mapping.entry(*community).or_insert_with(|| {
            new_graph.add_node((*community as f64, 0.0)).index()
        });
    }

    // Iterate over all edges in the original graph.
    for edge in graph.edge_references() {
        // Get the community assignments of the source and target nodes of the current edge.
        let source_community = community_assignments[&edge.source().index()];
        let target_community = community_assignments[&edge.target().index()];

        // If the source and target nodes belong to different communities...
        if source_community != target_community {
            // Get the new node indices that correspond to the source and target communities.
            let source_node = NodeIndex::new(mapping[&source_community]);
            let target_node = NodeIndex::new(mapping[&target_community]);
            // Get the weight of the current edge.
            let edge_weight = *edge.weight();

            // Check if there is already an edge between the new source and target nodes. If there is, update its weight. Otherwise, create a new edge with the original edge weight.
            let new_edge_weight = match new_graph.find_edge_undirected(source_node, target_node) {
                Some((edge_id, _)) => new_graph.edge_weight(edge_id).unwrap() + edge_weight,
                None => edge_weight,
            };

            new_graph.update_edge(source_node, target_node, new_edge_weight);
        }
    }

    // Create a new hash map to store the updated community assignments based on the new node indices.
    let mut new_community_assignments = HashMap::new();
    for (node, community) in community_assignments.iter() {
        new_community_assignments.insert(*node, mapping[community]);
    }

    // Return the new graph, new community assignments, and mapping between old and new communities as a tuple.
    (new_graph, new_community_assignments, mapping)
}




#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_graph() -> Graph<(f64, f64), f64, Undirected> {
        let mut graph = Graph::<(f64, f64), f64, Undirected>::new_undirected();
        let a = graph.add_node((0.0, 0.0));
        let b = graph.add_node((1.0, 0.0));
        let c = graph.add_node((1.0, 1.0));
        let d = graph.add_node((0.0, 1.0));

        graph.add_edge(a, b, 1.0);
        graph.add_edge(b, c, 1.0);
        graph.add_edge(c, d, 1.0);
        graph.add_edge(d, a, 1.0);
        graph.add_edge(a, c, 1.0);

        graph
    }

    #[test]
    fn test_initial_community_assignments() {
        let graph = create_test_graph();
        let community_assignments = initial_community_assignments(&graph);

        assert_eq!(
            community_assignments,
            HashMap::from_iter(vec![(0, 0), (1, 1), (2, 2), (3, 3)])
        );
    }

    #[test]
    fn test_leiden_communities() {
        let graph = create_test_graph();
        let community_assignments = leiden_communities(&graph);

        // Check if all nodes belong to the same community
        let unique_communities: std::collections::HashSet<usize> =
            community_assignments.values().cloned().collect();
        assert_eq!(unique_communities.len(), 1);

        // Check if community assignment is consistent across all nodes
        let first_community = *community_assignments.values().next().unwrap();
        for assignment in community_assignments.values() {
            assert_eq!(*assignment, first_community);
        }
    }
}


