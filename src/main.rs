mod csv_converter;  // Module for converting TXT to CSV
mod graph;          // Module for building the graph from CSV
mod algorithms;     // Module for BFS and Degree Centrality

use rand::prelude::{IteratorRandom, SliceRandom};  // For random sampling of graph nodes
use std::error::Error;
use std::collections::{HashMap, VecDeque};  // For BFS and storing the graph
use graph::build_graph_from_csv;  // Import function to build graph from CSV
use algorithms::{bfs, degree_centrality};  // Import BFS and Degree Centrality algorithms

/// Samples a connected subgraph from the graph by performing BFS from a random seed node.
/// It explores neighbors of the seed node and adds them until the subgraph reaches the target size.
///
/// # Arguments
/// * `graph` - The original graph represented as a `HashMap<usize, Vec<usize>>`.
/// * `max_nodes` - The desired number of nodes in the connected subgraph.
///
/// # Returns
/// This function returns a tuple containing:
/// * `seed_node` - The node from which the BFS started.
/// * `subgraph` - A `HashMap` representing the sampled subgraph.
fn sample_connected_subgraph(
    graph: &HashMap<usize, Vec<usize>>,
    max_nodes: usize,
) -> (usize, HashMap<usize, Vec<usize>>) {
    let mut rng = rand::thread_rng();
    // Choose a random seed node from the graph
    let &seed = graph
        .keys()
        .choose(&mut rng)
        .expect("graph must be non-empty");

    let mut visited = HashMap::new();  // Tracks visited nodes
    let mut queue = VecDeque::new();  // Queue for BFS
    visited.insert(seed, ());  // Mark the seed node as visited
    queue.push_back(seed);  // Start BFS from the seed node

    // Perform BFS to explore the graph until we have max_nodes in the subgraph
    while visited.len() < max_nodes {
        if let Some(node) = queue.pop_front() {  // Dequeue a node to explore
            if let Some(neighbors) = graph.get(&node) {  // Get neighbors of the current node
                for &nbr in neighbors {
                    if visited.len() >= max_nodes {
                        break;  // Stop if we reached the desired sample size
                    }
                    if visited.insert(nbr, ()).is_none() {  // Mark the neighbor as visited
                        queue.push_back(nbr);  // Add the neighbor to the queue
                    }
                }
            }
        } else {
            break;  // Exit if there are no more nodes to explore
        }
    }

    // Create the induced subgraph based on the visited nodes
    let nodes: Vec<usize> = visited.keys().copied().collect();
    let mut subgraph = HashMap::new();
    for &n in &nodes {
        let filtered = graph
            .get(&n)
            .unwrap_or(&Vec::new())
            .iter()
            .filter(|&&m| visited.contains_key(&m))  // Only include neighbors that are in the visited set
            .copied()
            .collect();
        subgraph.insert(n, filtered);  // Add the node and its neighbors to the subgraph
    }

    (seed, subgraph)  // Return the seed node and the sampled subgraph
}

/// Main function of the program. It performs the following tasks:
/// 1. Converts a TXT file containing graph data to a CSV file.
/// 2. Builds the graph from the CSV file.
/// 3. Samples a connected subgraph using BFS.
/// 4. Runs BFS on sample node pairs and computes Degree Centrality for the subgraph.
///
/// # Returns
/// This function returns a `Result<(), Box<dyn Error>>`, indicating success or failure.
fn main() -> Result<(), Box<dyn Error>> {
    // Paths to the input TXT file and output CSV file
    let txt = "web-Google.txt";
    let csv = "web-Google.csv";

    // Convert the TXT file to CSV format
    csv_converter::convert_txt_to_csv(txt, csv)?;

    // Build the full graph from the CSV file
    let graph = build_graph_from_csv(csv)?;
    println!("Full graph loaded: {} nodes", graph.len());

    // Sample a connected subgraph (20% of the nodes from the original graph)
    let sample_pct = 20.0;  // Increase sample size for better results
    let sample_size = ((graph.len() as f64) * sample_pct / 100.0).round() as usize;
    let (seed, subgraph) = sample_connected_subgraph(&graph, sample_size);
    println!(
        "Connected subgraph from seed {}: {} nodes",
        seed,
        subgraph.len()
    );

    // -- BFS: From seed to 3 random target nodes in the sampled subgraph ----------------------

    let mut rng = rand::thread_rng();
    let mut others: Vec<usize> = subgraph.keys().filter(|&&n| n != seed).copied().collect();
    others.shuffle(&mut rng);  // Randomly shuffle the target nodes
    let targets = others.into_iter().take(3).collect::<Vec<_>>();

    // Print the BFS results for the sampled node pairs
    println!("\n-- BFS from {} to targets --", seed);
    for tgt in targets {
        if let Some(path) = bfs(&subgraph, seed, tgt) {  // Find the shortest path between nodes
            println!("Path {}→{}: {:?}", seed, tgt, path);
        } else {
            println!("No path {}→{}", seed, tgt);
        }
    }

    // -- Degree Centrality: Top 10 nodes by degree -----------------------------------------
    println!("\n-- Degree Centrality (top 10) --");
    degree_centrality(&subgraph);  // Calculate and print degree centrality for the sampled subgraph

    Ok(())
}
