use std::collections::{HashMap, HashSet, VecDeque};

type NodeId = usize;
type Graph = HashMap<NodeId, HashSet<NodeId>>;

pub fn load_graph_from_file(file_path: &str) -> Result<Graph, std::io::Error> {
    use std::io::{self, BufRead};

    let file = std::fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut graph: Graph = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        if parts.len() == 2 {
            let from_node: NodeId = parts[0].parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            let to_node: NodeId = parts[1].parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            graph.entry(from_node).or_insert_with(HashSet::new).insert(to_node);
        }
    }

    Ok(graph)
}

pub fn in_degree_centrality(graph: &Graph) -> HashMap<NodeId, f64> {
    let mut centrality: HashMap<NodeId, f64> = HashMap::new();

    for &node in graph.keys() {
        let in_degree = graph.values().filter(|neighbors| neighbors.contains(&node)).count() as f64;
        centrality.insert(node, in_degree);
    }

    let total_nodes = graph.len() as f64;
    centrality.iter_mut().for_each(|(_, value)| *value /= total_nodes);

    centrality
}

pub fn out_degree_centrality(graph: &Graph) -> HashMap<NodeId, f64> {
    let mut centrality: HashMap<NodeId, f64> = HashMap::new();

    for (&node, neighbors) in graph.iter() {
        let out_degree = neighbors.len() as f64;
        centrality.insert(node, out_degree);
    }

    let total_nodes = graph.len() as f64;
    centrality.iter_mut().for_each(|(_, value)| *value /= total_nodes);

    centrality
}

pub fn bfs_shortest_path(graph: &Graph, start: NodeId, end: NodeId) -> Option<Vec<NodeId>> {
    let mut visited: HashSet<NodeId> = HashSet::new();
    let mut queue: VecDeque<Vec<NodeId>> = VecDeque::new();
    queue.push_back(vec![start]);

    while let Some(path) = queue.pop_front() {
        let node = *path.last().unwrap();

        if node == end {
            return Some(path);
        }

        if visited.contains(&node) {
            continue;
        }

        visited.insert(node);

        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                let mut new_path = path.clone();
                new_path.push(neighbor);
                queue.push_back(new_path);
            }
        }
    }

    None
}


pub fn mean_degree(graph: &Graph) -> f64 {
    let total_nodes = graph.len() as f64;
    let total_edges: usize = graph.values().map(|neighbors| neighbors.len()).sum();

    total_edges as f64 / total_nodes
}

pub fn degree_variance(graph: &Graph) -> f64 {
    let mean = mean_degree(&graph);
    let total_nodes = graph.len() as f64;

    let variance: f64 = graph.values().map(|neighbors| (neighbors.len() as f64 - mean).powi(2)).sum();
    variance / total_nodes
}
/// Calculate the global clustering coefficient for the graph.
pub fn global_clustering_coefficient(graph: &Graph) -> f64 {
    let mut total_coefficient = 0.0;
    let mut nodes_with_neighbors = 0;

    for (_, neighbors) in graph {
        let num_neighbors = neighbors.len();

        if num_neighbors >= 2 {
            let mut triangles = 0;

            for neighbor1 in neighbors {
                for neighbor2 in neighbors {
                    if neighbor1 != neighbor2 && graph.contains_key(&neighbor1) {
                        let neighbor1_neighbors = &graph[neighbor1];

                        if neighbor1_neighbors.contains(neighbor2) {
                            triangles += 1;
                        }
                    }
                }
            }

            let local_coefficient = triangles as f64 / (num_neighbors * (num_neighbors - 1)) as f64;
            total_coefficient += local_coefficient;
            nodes_with_neighbors += 1;
        }
    }

    if nodes_with_neighbors > 0 {
        total_coefficient / nodes_with_neighbors as f64
    } else {
        0.0
    }
}