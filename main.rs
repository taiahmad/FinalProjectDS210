use std::io;

mod graph;

fn main() -> io::Result<()> {
    let file_path = "p2p-Gnutella04.txt";

    let graph = graph::load_graph_from_file(file_path)?;

    // In-Degree Centrality
    let in_degree_centrality = graph::in_degree_centrality(&graph);
    println!("In-Degree Centrality (Top 5):");
    print_top_entries(&in_degree_centrality, 5);

    // Out-Degree Centrality
    let out_degree_centrality = graph::out_degree_centrality(&graph);
    println!("Out-Degree Centrality (Top 5):");
    print_top_entries(&out_degree_centrality, 5);

    // BFS Shortest Path between nodes 0 and 1
    if let Some(shortest_path) = graph::bfs_shortest_path(&graph, 4780, 5049) {
        println!("Shortest Path between nodes 4780 and 5049: {:?}", shortest_path);
    } else {
        println!("No path between nodes 0 and 1.");
    }

    // Mean Degree
    let mean_degree = graph::mean_degree(&graph);
    println!("Mean Degree: {:.2}", mean_degree);

    // Variance of Degree
    let degree_variance = graph::degree_variance(&graph);
    println!("Variance of Degree: {:.2}", degree_variance);

    // Calculate and print the global clustering coefficient
    let global_clustering_coefficient = graph::global_clustering_coefficient(&graph);
    println!("Global Clustering Coefficient: {:.4}", global_clustering_coefficient);

    Ok(())
}

fn print_top_entries<K, V>(map: &std::collections::HashMap<K, V>, limit: usize)
where
    K: std::fmt::Debug + Ord,
    V: std::fmt::Debug,
{
    for (key, value) in map.iter().take(limit) {
        println!("{:?}: {:?}", key, value);
    }
}
