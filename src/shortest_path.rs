// shortest_path.rs

use petgraph::Graph;
use petgraph::algo::dijkstra;
use petgraph::Undirected;
use petgraph::graph::NodeIndex;
use petgraph::visit::Bfs;
use std::collections::HashMap;
use petgraph::visit::EdgeRef;
use rand::seq::SliceRandom;
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::collections::BinaryHeap;
use crate::HashSet;

// Landmark-based approach

// Take a reference to a generic `Graph` with node properties of `(f64, f64)` type, edge weights of type `f64`, and undirected edges; and an integer `k` specifying the number of landmarks to select. The function uses a random approach to select `k` landmarks from the nodes in the graph and returns them as a vector of `NodeIndex` values.
pub fn select_landmarks(graph: &Graph<(f64, f64), f64, Undirected>, k: usize) -> Vec<NodeIndex> {
    // Initialize a random number generator.
    let mut rng = rand::thread_rng();

    // Get the indices of all nodes in the graph and shuffle them randomly.
    let mut landmarks: Vec<NodeIndex> = graph.node_indices().collect();
    landmarks.shuffle(&mut rng);

    // Truncate the shuffled indices to select the first `k` indices as the landmarks and return them as a vector.
    landmarks.truncate(k);
    landmarks
}

// Take a reference to a generic `Graph` with node properties of `(f64, f64)` type, edge weights of type `f64`, and undirected edges; and a slice of `NodeIndex` values representing the landmarks in the graph. The function computes the shortest distances from each landmark to every other node in the graph using Dijkstra's algorithm and returns the distances as a nested `HashMap`, where the outer key represents the landmark node and the inner key represents the target node, and the value is the shortest distance between them.
pub fn precompute_landmark_distances(
    graph: &Graph<(f64, f64), f64, Undirected>,
    landmarks: &[NodeIndex],
) -> HashMap<NodeIndex, HashMap<NodeIndex, f64>> {
    // Initialize a new empty `HashMap` to store the distances from each landmark to every other node in the graph.
    let mut landmark_distances: HashMap<NodeIndex, HashMap<NodeIndex, f64>> = HashMap::new();

    // Iterate through each landmark in the input slice and compute the shortest distances from the landmark to every other node in the graph using Dijkstra's algorithm.
    for &landmark in landmarks {
        let distances = dijkstra(graph, landmark, None, |e| *e.weight());

        // Insert the distances from the landmark to every other node in the graph into the `landmark_distances` hashmap.
        landmark_distances.insert(landmark, distances);
    }

    // Return the `landmark_distances` hashmap.
    landmark_distances
}

// Take as input the starting node index `start_node`, the ending node index `end_node`, a reference to a nested `HashMap` of `NodeIndex` to `f64` values representing the precomputed distances from each landmark to every other node in the graph, and a constant `alpha` of type `f64`. The function returns an approximation of the shortest path distance between the start and end nodes using the landmark-based approach, where `alpha` is a tuning parameter that determines the trade-off between speed and accuracy of the approximation.
pub fn approximate_shortest_path(
    start_node: NodeIndex,
    end_node: NodeIndex,
    landmark_distances: &HashMap<NodeIndex, HashMap<NodeIndex, f64>>,
    alpha: f64,
) -> f64 {
    // Initialize a variable to store the minimum distance seen so far to infinity.
    let mut min_distance = f64::INFINITY;

    // Iterate through each landmark in the precomputed distances hashmap and compute the total distance from the start node to the end node through the current landmark. If the total distance is less than or equal to the current minimum distance multiplied by the tuning parameter alpha, update the minimum distance.
    for (_, landmark_dist) in landmark_distances.iter() {
        if let (Some(dist_start), Some(dist_end)) = (
            landmark_dist.get(&start_node),
            landmark_dist.get(&end_node),
        ) {
            let distance = dist_start + dist_end;
            if distance <= alpha * min_distance {
                min_distance = distance;
            }
        }
    }

    // Return the final minimum distance seen.
    min_distance
}


// Have three fields: a `f64` value, a `NodeIndex`, and a nested `HashMap` of `NodeIndex` values.
#[derive(Debug, PartialEq)]
struct QueueItem(f64, NodeIndex, HashMap<NodeIndex, NodeIndex>);

// Implement the `Eq` trait for `QueueItem`.
impl Eq for QueueItem {}

// Implement the `PartialOrd` trait for `QueueItem`.
impl PartialOrd for QueueItem {
    // Define a function called `partial_cmp` that takes a reference to another `QueueItem` struct and returns an `Option` of type `Ordering`.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Call the `partial_cmp` method on the `f64` value of the `other` `QueueItem` and compare it to the `f64` value of `self`. This will return an `Option` of type `Ordering`.
        other.0.partial_cmp(&self.0)
    }
}

// Implement the `Ord` trait for `QueueItem`.
impl Ord for QueueItem {
    // Define a function called `cmp` that takes another `QueueItem` struct and returns an `Ordering`.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Call the `partial_cmp` method defined in the `PartialOrd` implementation and use `unwrap_or` to return `Equal` if the result is `None`.
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}


// Finds the shortest paths between two nodes in a graph using landmarks
// Returns a vector of tuples containing the weight of each path and a map of the nodes in the path and their predecessors
pub fn find_shortest_paths(
    graph: &Graph<(f64, f64), f64, Undirected>,
    start_node: NodeIndex,
    end_node: NodeIndex,
    landmark_distances: &HashMap<NodeIndex, HashMap<NodeIndex, f64>>,
    num_paths: usize,
) -> Vec<(f64, HashMap<NodeIndex, NodeIndex>)> {
    // Set the value of alpha, which controls the accuracy of the algorithm
    let alpha = 3.0; // Adjust this value based on your needs

    // Compute an approximate shortest path between the start and end nodes
    let approx_distance = approximate_shortest_path(start_node, end_node, landmark_distances, alpha);

    // Initialize the visited set and priority queue
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push(QueueItem(0.0, start_node, HashMap::new()));

    // Initialize the vector to store the shortest paths
    let mut shortest_paths = Vec::new();

    // While the priority queue is not empty
    while let Some(QueueItem(weight, current_node, path)) = queue.pop() {
        // If the current node is the end node, add the path to the shortest paths vector
        if current_node == end_node {
            shortest_paths.push((weight, path.clone()));

            // If the desired number of paths have been found, stop the loop
            if shortest_paths.len() >= num_paths {
                break;
            }
        }

        // If the current node has already been visited, skip to the next iteration
        if visited.contains(&current_node) {
            continue;
        }

        // Add the current node to the visited set
        visited.insert(current_node);

        // For each outgoing edge from the current node
        for edge in graph.edges(current_node) {
            let neighbor = edge.target();
            if !visited.contains(&neighbor) {
                // Calculate the weight of the new path and the approximate distance from the neighbor to the end node
                let new_weight = weight + *edge.weight();
                let approx_neighbor_distance =
                    approximate_shortest_path(neighbor, end_node, landmark_distances, alpha);

                // If the total estimated distance from the start to the end through the neighbor is less than alpha times the approximate distance
                if new_weight + approx_neighbor_distance <= alpha * approx_distance {
                    // Create a new path and add it to the priority queue
                    let mut new_path = path.clone();
                    new_path.insert(current_node, neighbor);
                    queue.push(QueueItem(new_weight, neighbor, new_path));
                }
            }
        }
    }
    shortest_paths
}




#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::EdgeIndex;

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
        graph.add_edge(a, c, 1.41);
        graph.add_edge(b, d, 1.41);

        graph
    }

    #[test]
    fn test_select_landmarks() {
        let graph = create_test_graph();
        let landmarks = select_landmarks(&graph, 2);

        assert_eq!(landmarks.len(), 2);
    }

    #[test]
    fn test_precompute_landmark_distances() {
        let graph = create_test_graph();
        let landmarks = select_landmarks(&graph, 2);
        let landmark_distances = precompute_landmark_distances(&graph, &landmarks);

        assert_eq!(landmark_distances.len(), 2);
    }

    #[test]
    fn test_approximate_shortest_path() {
        let graph = create_test_graph();
        let landmarks = select_landmarks(&graph, 2);
        let landmark_distances = precompute_landmark_distances(&graph, &landmarks);

        let start_node = NodeIndex::new(0);
        let end_node = NodeIndex::new(2);

        let alpha = 3.0;
        let approx_distance = approximate_shortest_path(start_node, end_node, &landmark_distances, alpha);

        assert!(approx_distance >= 1.41 && approx_distance <= alpha * 1.41);
    }

    #[test]
    fn test_find_shortest_paths() {
        let graph = create_test_graph();
        let landmarks = select_landmarks(&graph, 2);
        let landmark_distances = precompute_landmark_distances(&graph, &landmarks);

        let start_node = NodeIndex::new(0);
        let end_node = NodeIndex::new(2);
        let num_paths = 1;

        let shortest_paths = find_shortest_paths(&graph, start_node, end_node, &landmark_distances, num_paths);

        assert_eq!(shortest_paths.len(), num_paths);
        assert_eq!(shortest_paths[0].0, 1.41);
    }
}



