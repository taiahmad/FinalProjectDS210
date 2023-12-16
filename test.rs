// test.rs

use crate::graph::{Graph, NodeId};
use std::collections::HashSet;

#[test]
fn test_load_graph_from_file() {
    // Create a temporary file for testing
    let file_path = "test_graph.txt";
    let graph_data = "0\t1\n1\t2\n2\t3\n3\t4\n";

    std::fs::write(file_path, graph_data).expect("Failed to write test graph file");

    let result = Graph::load_graph_from_file(file_path);
    assert!(result.is_ok(), "Failed to load graph from file: {:?}", result.err());
}

#[test]
fn test_shortest_path() {
    let mut graph = Graph::new();
    graph.add_edge(0, 1);
    graph.add_edge(1, 2);
    graph.add_edge(2, 3);
    graph.add_edge(3, 4);

    let path = graph.shortest_path(0, 4);
    assert_eq!(path, Some(vec![0, 1, 2, 3, 4]), "Incorrect shortest path");
}

#[test]
fn test_clustering_coefficient() {
    let mut graph = Graph::new();
    graph.add_edge(0, 1);
    graph.add_edge(1, 2);
    graph.add_edge(2, 3);
    graph.add_edge(3, 4);

    let coefficient = graph.clustering_coefficient(2);
    assert_eq!(coefficient, 0.0, "Incorrect clustering coefficient for Node 2");
}

#[test]
fn test_mean_degree() {
    let mut graph = Graph::new();
    graph.add_edge(0, 1);
    graph.add_edge(1, 2);
    graph.add_edge(2, 3);
    graph.add_edge(3, 4);

    let mean_degree = graph.mean_degree();
    assert_eq!(mean_degree, 2.0, "Incorrect mean degree");
}