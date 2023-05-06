// main.rs

mod data;
mod centrality;
mod centrality_analysis;
mod shortest_path;
mod pagerank;
mod network_analysis;
mod leiden;
mod assort;
mod cpa;
mod bed;
mod cca;

use petgraph::graphmap::GraphMap;
use petgraph::graph::Graph;
use petgraph::Undirected;
use crate::centrality::{degree_centrality};
use crate::centrality_analysis::analyze_centrality;
use petgraph::graph::NodeIndex;
use crate::shortest_path::find_shortest_paths;
use crate::pagerank::pagerank;
use crate::network_analysis::{degree_distribution, clustering_coefficient, network_density};
use std::fs::File;
use std::io::Write;
use serde_json;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use crate::shortest_path::select_landmarks;
use crate::shortest_path::precompute_landmark_distances;
use crate::assort::calculate_assortativity_coefficient;
use petgraph::algo::connected_components;
use petgraph::visit::FilterNode;
use std::fs;
use std::collections::HashSet;
use rand::Rng;
use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
struct ShortestPath {
    index: usize,
    weight: f64,
    path: Vec<usize>,
}


// This function loads a graph from a file in text format
fn load_graph_from_txt(file_path: &str) -> Graph<(f64, f64), f64, petgraph::Undirected> {
    // Open the file and create a buffered reader
    let file = File::open(file_path).expect("Unable to open the file");
    let reader = BufReader::new(file);

    // Create an empty graph
    let mut graph = Graph::<(f64, f64), f64, petgraph::Undirected>::new_undirected();

    // Create a HashMap to keep track of node indices
    let mut node_indices = HashMap::new();

    // Iterate over the lines in the file
    for line in reader.lines() {
        // Parse the line and split it into two node identifiers
        let line = line.expect("Unable to read line");
        let nodes: Vec<&str> = line.trim().split_whitespace().collect();

        // If the line contains two nodes, add them to the graph
        if nodes.len() == 2 {
            // Parse the node identifiers into integers
            let from_node: usize = nodes[0].parse().unwrap();
            let to_node: usize = nodes[1].parse().unwrap();

            // Get the indices of the nodes, adding them to the graph if necessary
            let from_node_index = *node_indices.entry(from_node).or_insert_with(|| graph.add_node((from_node as f64, 0.0)));
            let to_node_index = *node_indices.entry(to_node).or_insert_with(|| graph.add_node((to_node as f64, 0.0)));

            // Add an edge between the nodes with weight 1.0
            graph.add_edge(from_node_index, to_node_index, 1.0);
        }
    }

    // Return the loaded graph
    graph
}


fn get_start_end_nodes(graph: &Graph<(f64, f64), f64, Undirected>) -> (NodeIndex, NodeIndex) {
    // Get the number of nodes in the graph
    let num_nodes = graph.node_count();

    // Choose a random start node
    let start_node = rand::thread_rng().gen_range(0..num_nodes);

    // Convert the start node index to a `NodeIndex`
    let start_node_index = NodeIndex::new(start_node);

    // Perform a breadth-first search to find all nodes connected to the start node
    let mut connected_component = Vec::new();
    let mut bfs = petgraph::visit::Bfs::new(graph, start_node_index);

    while let Some(node) = bfs.next(graph) {
        connected_component.push(node);
    }

    // Choose a random node from the connected component as the end node
    let end_node_index = connected_component.choose(&mut rand::thread_rng()).unwrap().to_owned();

    // Return the start and end node indices as a tuple
    (start_node_index, end_node_index)
}



fn main() {
    let graph = data::read_and_preprocess_data("data/roadNet-CA.txt");

    // data.rs, printing number of nodes and edges
    println!("Number of nodes: {}", graph.node_count());
    println!("Number of edges: {}", graph.edge_count());

    // Centrality Analysis
    let degree_centrality = analyze_centrality(&graph);

    // Serialize the degree_centrality to a JSON string
    let json_output = serde_json::to_string_pretty(&degree_centrality)
        .expect("Failed to serialize degree centrality to JSON");

    // Write the JSON output to a file
    let mut file = File::create("sample_degree_centrality.json")
        .expect("Failed to create output file");
    file.write_all(json_output.as_bytes())
        .expect("Failed to write JSON to output file");

    println!("Degree centrality written to sample_degree_centrality.json");

    // Deserialize the JSON file back into a HashMap
    let json_data = fs::read_to_string("sample_degree_centrality.json")
        .expect("Unable to read the file");
    let degree_centrality: HashMap<usize, f64> = serde_json::from_str(&json_data)
        .expect("Unable to deserialize JSON data");

    // Find the top nodes with the highest degree centrality
    let mut top_nodes: Vec<(&usize, &f64)> = degree_centrality.iter().collect();
    top_nodes.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    top_nodes.truncate(10);

    println!("Top nodes with the highest degree centrality:");
    for (i, (node, centrality)) in top_nodes.iter().enumerate() {
        println!("Node {}: id={}, degree centrality={}", i + 1, node, centrality);
    }

    // Convert the UnGraphMap<usize, f64> to Graph<(f64, f64), f64, Undirected>
    let mut graph_with_coordinates: Graph<(f64, f64), f64, Undirected> = Graph::default();
    let node_map: std::collections::HashMap<usize, _> = graph.nodes().map(|n| {
        (n, graph_with_coordinates.add_node((n as f64, n as f64)))
    }).collect();

    for (a, b, weight) in graph.all_edges() {
        let a_node = node_map[&a];
        let b_node = node_map[&b];
        graph_with_coordinates.add_edge(a_node, b_node, *weight);
    }

    // Network analysis
    let distribution = degree_distribution(&graph_with_coordinates);
    let coefficient = clustering_coefficient(&graph_with_coordinates);
    let density = network_density(&graph_with_coordinates);

    println!("Degree distribution: {:?}", distribution);
    println!("Clustering coefficient: {:?}", coefficient);
    println!("Network density: {:?}", density);

    // PageRank
    let pagerank_results = pagerank(&graph_with_coordinates, 0.85, 100);
    println!("PageRank results:");
    for (node, rank) in pagerank_results.iter().take(10) {
        println!("Node: {:?}, Rank: {}", node, rank);
    }

    // Assortativity
    let assortativity_coefficient = calculate_assortativity_coefficient(&graph_with_coordinates);
    println!("Assortativity Coefficient: {}", assortativity_coefficient);

    // Leiden Implementation
    let community_assignments = leiden::leiden_communities(&graph_with_coordinates);

    // Serialize the community_assignments to a JSON string
    let json_output = serde_json::to_string_pretty(&community_assignments)
        .expect("Failed to serialize community assignments to JSON");

    // Write the JSON output to a file
    let mut file = File::create("sample_leiden_output.json")
        .expect("Failed to create output file");
    file.write_all(json_output.as_bytes())
        .expect("Failed to write JSON to output file");

    println!("Leiden community assignments written to sample_leiden_output.json");

    // Deserialize the JSON file back into a HashMap
    let json_data = fs::read_to_string("sample_leiden_output.json")
        .expect("Unable to read the file");
    let community_assignments: HashMap<usize, usize> = serde_json::from_str(&json_data)
        .expect("Unable to deserialize JSON data");

    // Create a new HashMap to store community members
    let mut community_members: HashMap<usize, Vec<usize>> = HashMap::new();

    for (node, community) in community_assignments.iter() {
        community_members.entry(*community).or_insert_with(Vec::new).push(*node);
    }

    // Sort the community_members by community size
    let mut communities_sorted: Vec<(usize, Vec<usize>)> = community_members.into_iter().collect();
    communities_sorted.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

    // Print a summary of communities and their sizes
    println!("Number of communities: {}", communities_sorted.len());
    println!("Top communities and their sizes:");

    // If you want to limit the number of communities displayed in the summary, you can change the loop like this:
    let num_top_communities = 10; // Change this to the number of top communities you want to display
    for (i, (community_id, members)) in communities_sorted.iter().take(num_top_communities).enumerate() {
        println!("Community {}: id={}, size={}", i + 1, community_id, members.len());
    }

    // Core-Periphery Analysis
    let degree_threshold = 10; // You can adjust this threshold based on your analysis
    let (core_nodes, periphery_nodes) = cpa::core_periphery_analysis(&graph_with_coordinates, degree_threshold);

    // Print the results of the core-periphery analysis
    println!("Number of core nodes: {}", core_nodes.len());
    println!("Number of periphery nodes: {}", periphery_nodes.len());

    // Sort the core and periphery nodes by degree and take the top ten
    let mut sorted_core_nodes: Vec<_> = core_nodes.iter().collect();
    sorted_core_nodes.sort_by_key(|node| graph_with_coordinates.neighbors(**node).count());
    let top_core_nodes = sorted_core_nodes.iter().rev().take(10).collect::<Vec<_>>();

    let mut sorted_periphery_nodes: Vec<_> = periphery_nodes.iter().collect();
    sorted_periphery_nodes.sort_by_key(|node| graph_with_coordinates.neighbors(**node).count());
    let top_periphery_nodes = sorted_periphery_nodes.iter().rev().take(10).collect::<Vec<_>>();

    // Print the top ten core and periphery nodes
    println!("Top 10 core nodes: {:?}", top_core_nodes);
    println!("Top 10 periphery nodes: {:?}", top_periphery_nodes);

    // Define a custom threshold value for the number of bridge edges to display
    let bridge_edges_threshold: usize = 10; // You can change this value later

    // Find bridge edges
    let bridge_edges = bed::find_bridge_edges(&graph_with_coordinates);
    println!("Number of bridge edges: {}", bridge_edges.len());

    // Convert the HashSet to a Vec
    let bridge_edges_vec: Vec<_> = bridge_edges.into_iter().collect();

    // Print bridge edges up to the threshold value
    println!("Bridge edges (up to {}):", bridge_edges_threshold);
    for (i, edge) in bridge_edges_vec.iter().take(bridge_edges_threshold).enumerate() {
        println!("Bridge edge {}: {:?}", i + 1, edge);
    }

    // Call the analyze_connected_components function with the graph_with_coordinates
    let (num_components, _) = cca::analyze_connected_components(&graph_with_coordinates);

    // Print the number of connected components
    println!("Number of connected components: {}", num_components);

    // Select landmarks
    let num_landmarks = 10; // You can adjust this number based on your graph size and desired performance
    let landmarks = select_landmarks(&graph_with_coordinates, num_landmarks);

    // Precompute landmark distances
    let landmark_distances = precompute_landmark_distances(&graph_with_coordinates, &landmarks);

    // Get starting and ending nodes
    let (start_node, end_node) = get_start_end_nodes(&graph_with_coordinates);

    // Select landmarks
    let num_landmarks = 10; // You can adjust this number based on your graph size and desired performance
    let landmarks = select_landmarks(&graph_with_coordinates, num_landmarks);

    // Precompute landmark distances
    let landmark_distances = precompute_landmark_distances(&graph_with_coordinates, &landmarks);

    // Get starting and ending nodes
    let (start_node, end_node) = get_start_end_nodes(&graph_with_coordinates);

    // Shortest Path
    let shortest_paths = find_shortest_paths(&graph_with_coordinates, start_node, end_node, &landmark_distances, 10);
    match shortest_paths.len() {
        0 => {
            println!("No path found between the starting and ending nodes.");
        }
        _ => {
            let mut shortest_paths_output = Vec::new();
            for (i, (path_weight, path)) in shortest_paths.iter().enumerate() {
                let mut current_node = start_node;
                let mut path_indices = vec![current_node.index()];
                while current_node != end_node {
                    match path.get(&current_node) {
                        Some(next_node) => {
                            current_node = *next_node;
                            path_indices.push(current_node.index());
                        }
                        None => {
                            break;
                        }
                    }
                }
                shortest_paths_output.push(ShortestPath {
                    index: i + 1,
                    weight: *path_weight,
                    path: path_indices,
                });
            }

            let json_output = serde_json::to_string_pretty(&shortest_paths_output).unwrap();

            let mut file = File::create("sample_shortest_paths.json").expect("Unable to create file");
            file.write_all(json_output.as_bytes()).expect("Unable to write data");
            println!("Shortest paths saved to sample_shortest_paths.json");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_graph_from_txt() {
        let graph = load_graph_from_txt("test_data/test_graph.txt");

        assert_eq!(graph.node_count(), 4);
        assert_eq!(graph.edge_count(), 3);
    }

    #[test]
    fn test_get_start_end_nodes() {
        let graph = load_graph_from_txt("test_data/test_graph.txt");
        let (start_node, end_node) = get_start_end_nodes(&graph);

        assert!(graph.node_indices().any(|n| n == start_node));
        assert!(graph.node_indices().any(|n| n == end_node));
    }
}
