// export.rs

use petgraph::visit::EdgeRef;
use petgraph::Graph;
use petgraph::Undirected;
use serde::Serialize;
use serde_json::json;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
struct NodeData {
    id: String,
    label: String,
    position: Position,
}

#[derive(Serialize)]
struct Position {
    x: f64,
    y: f64,
}

#[derive(Serialize)]
struct EdgeData {
    from: String,
    to: String,
}

pub fn export_graph_to_json(graph: &Graph<(f64, f64), f64, Undirected>, output_path: &str) {
    let nodes: Vec<NodeData> = graph
        .node_indices()
        .map(|i| {
            let (x, y) = graph[i];
            NodeData {
                id: format!("n{}", i.index()),
                label: format!("Node {}", i.index()),
                position: Position { x, y },
            }
        })
        .collect();

    let edges: Vec<EdgeData> = graph
        .edge_references()
        .map(|e| EdgeData {
            from: format!("n{}", e.source().index()),
            to: format!("n{}", e.target().index()),
        })
        .collect();

    let elements = json!({
        "nodes": nodes,
        "edges": edges,
    });

    let mut file = File::create(output_path).expect("Unable to create file");
    file.write_all(elements.to_string().as_bytes())
        .expect("Unable to write data");
}



#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::NodeIndex;
    use std::fs;
    use std::path::Path;

    const OUTPUT_PATH: &str = "test_export.json";

    fn create_test_graph() -> Graph<(f64, f64), f64, Undirected> {
        let mut graph = Graph::<(f64, f64), f64, Undirected>::new_undirected();
        let a = graph.add_node((0.0, 0.0));
        let b = graph.add_node((1.0, 1.0));
        graph.add_edge(a, b, 1.0);

        graph
    }

    #[test]
    fn test_export_graph_to_json() {
        let graph = create_test_graph();
        export_graph_to_json(&graph, OUTPUT_PATH);

        assert!(Path::new(OUTPUT_PATH).exists());

        let file_contents = fs::read_to_string(OUTPUT_PATH).expect("Unable to read file");
        let file_json: serde_json::Value =
            serde_json::from_str(&file_contents).expect("Unable to parse JSON");

        let nodes = file_json.get("nodes").expect("No nodes found in JSON");
        let edges = file_json.get("edges").expect("No edges found in JSON");

        assert_eq!(nodes.as_array().unwrap().len(), 2);
        assert_eq!(edges.as_array().unwrap().len(), 1);

        let first_node = &nodes[0];
        assert_eq!(first_node.get("id").unwrap(), "n0");
        assert_eq!(first_node.get("label").unwrap(), "Node 0");
        assert_eq!(first_node.get("position").unwrap().get("x").unwrap(), 0.0);
        assert_eq!(first_node.get("position").unwrap().get("y").unwrap(), 0.0);

        let first_edge = &edges[0];
        assert_eq!(first_edge.get("from").unwrap(), "n0");
        assert_eq!(first_edge.get("to").unwrap(), "n1");

        fs::remove_file(OUTPUT_PATH).expect("Unable to delete test file");
    }
}

