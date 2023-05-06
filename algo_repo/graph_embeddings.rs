// graph_embeddings.rs

use petgraph::Graph;
use petgraph::Undirected;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use petgraph::graph::NodeIndex;
use rand::rngs::ThreadRng;
use rand::distributions::WeightedIndex;
use std::thread::current;
use rand::distributions::Distribution;


pub fn node2vec(
    graph: &Graph<(f64, f64), f64, Undirected>,
    dimensions: usize,
    walk_length: usize,
    num_walks: usize,
    p: f64,
    q: f64,
    learning_rate: f64,
    epochs: usize,
) -> HashMap<usize, Vec<f64>> {
    let mut embeddings: HashMap<usize, Vec<f64>> = HashMap::new();

    // Initialize random embeddings for each node
    for node in graph.node_indices() {
        let mut rng = rand::thread_rng();
        let embedding: Vec<f64> = (0..dimensions)
            .map(|_| rng.gen_range(-1.0..1.0))
            .collect();
        embeddings.insert(node.index(), embedding);
    }

    // Generate random walks
    let walks = random_walks(graph, num_walks, walk_length, p, q);

    for _ in 0..epochs {
        for walk in &walks {
            for (i, node) in walk.iter().enumerate() {
                let context = &walk[i.saturating_sub(1)..i]
                    .iter()
                    .chain(walk.iter().skip(i + 1).take(1))
                    .collect::<Vec<_>>();

                let node_embedding = &embeddings[&node.index()];

                // Calculate gradients
                let mut gradients: Vec<(NodeIndex, Vec<f64>)> = vec![];
                for context_node in context {
                    let context_embedding = &embeddings[&context_node.index()];
                    let dot_product = node_embedding.iter().zip(context_embedding.iter()).map(|(a, b)| a * b).sum::<f64>();
                    let gradient = 1.0 / (1.0 + (-dot_product).exp());

                    let context_gradient: Vec<f64> = context_embedding.iter().map(|&value| value * gradient).collect();
                    gradients.push((**context_node, context_gradient));
                }

                // Update embeddings
                for (context_node, gradient) in gradients.iter() {
                    let context_embedding = embeddings.get_mut(&context_node.index()).unwrap();
                    for (component, update) in context_embedding.iter_mut().zip(gradient) {
                        *component += learning_rate * update;
                    }
                }
            }
        }
    }

    embeddings
}




pub fn random_walk(
    graph: &Graph<(f64, f64), f64, Undirected>,
    start_node: NodeIndex,
    walk_length: usize,
    p: f64,
    q: f64,
    rng: &mut ThreadRng,
) -> Vec<NodeIndex> {
    let mut walk = Vec::with_capacity(walk_length);
    walk.push(start_node);
    for _ in 1..walk_length {
        let node = walk.last().unwrap();
        let neighbors = graph.neighbors(*node).collect::<Vec<_>>();
        if neighbors.is_empty() {
            break;
        }

        let previous_node = if walk.len() > 1 { Some(walk[walk.len() - 2]) } else { None };

        let next_node = if let Some(prev) = previous_node {
            let weights = create_weights(graph, &neighbors, prev, p, q);
            let dist = WeightedIndex::new(&weights).expect("Failed to create weighted index");
            neighbors[dist.sample(rng)]
        } else {
            neighbors
                .choose(rng)
                .copied()
                .expect("Failed to choose from non-empty slice")
        };


        walk.push(next_node);
    }

    walk
}

pub fn random_walks(
    graph: &Graph<(f64, f64), f64, Undirected>,
    walk_length: usize,
    num_walks: usize,
    p: f64,
    q: f64,
) -> Vec<Vec<NodeIndex>> {
    let mut walks = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..num_walks {
        for node in graph.node_indices() {
            let walk = random_walk(graph, node, walk_length, p, q, &mut rng);
            walks.push(walk);
        }
    }

    walks
}






fn transition_probabilities(
    graph: &Graph<(f64, f64), f64, Undirected>,
    neighbors: &[NodeIndex],
    previous: usize,
    p: f64,
    q: f64,
) -> HashMap<usize, f64> {
    let mut probabilities = HashMap::new();
    let mut sum_probabilities = 0.0;

    for &neighbor in neighbors {
        let edge_index = graph.find_edge(
            petgraph::graph::NodeIndex::new(previous),
            petgraph::graph::NodeIndex::new(neighbor.index()),
        );
        let weight = match edge_index {
            Some(index) => graph.edge_weight(index).unwrap_or(&1.0),
            None => &1.0,
        };

        let probability = {
            if neighbor == petgraph::graph::NodeIndex::new(previous) {
                *weight / p
            } else if graph.contains_edge(
                petgraph::graph::NodeIndex::new(previous),
                petgraph::graph::NodeIndex::new(neighbor.index()),
            ) {
                *weight
            } else {
                *weight / q
            }
        };

        sum_probabilities += probability;
        probabilities.insert(neighbor.index(), probability);
    }

    for (_, value) in probabilities.iter_mut() {
        *value /= sum_probabilities;
    }

    probabilities
}



fn create_weights(
    graph: &Graph<(f64, f64), f64, Undirected>,
    neighbors: &[NodeIndex],
    previous: NodeIndex,
    p: f64,
    q: f64,
) -> Vec<f64> {
    let probabilities = transition_probabilities(graph, neighbors, previous.index(), p, q);
    neighbors.iter().map(|&neighbor| probabilities[&neighbor.index()]).collect()
}




fn sgd(
    embeddings: &mut HashMap<usize, Vec<f64>>,
    node: usize,
    context: &[&usize],
    learning_rate: f64,
) {
    let node_embedding = &embeddings[&node];
    let mut gradient_sum = vec![0.0; node_embedding.len()];
    let mut updates = Vec::new();

    for &context_node in context {
        let context_embedding = &embeddings[context_node];
        let dot_product = node_embedding
            .iter()
            .zip(context_embedding.iter())
            .map(|(a, b)| a * b)
            .sum::<f64>();

        let mut gradient = vec![0.0; node_embedding.len()];
        for i in 0..node_embedding.len() {
            gradient[i] = context_embedding[i] - node_embedding[i] * dot_product;
            gradient_sum[i] += gradient[i];
        }

        updates.push((context_node, gradient));
    }

    for (context_node, update) in updates {
        let context_embedding = embeddings.get_mut(context_node).unwrap();
        for i in 0..context_embedding.len() {
            context_embedding[i] += learning_rate * update[i];
        }
    }

    let node_embedding = embeddings.get_mut(&node).unwrap();
    for i in 0..node_embedding.len() {
        node_embedding[i] += learning_rate * gradient_sum[i];
    }
}



