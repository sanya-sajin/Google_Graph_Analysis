use std::collections::{HashMap, HashSet, VecDeque};
// Module containing BFS and Degree Centrality code 
/// BFS: Find the shortest path between two nodes in an unweighted graph.
/// 
/// This function uses a **breadth-first search (BFS)** algorithm to find the **shortest path** between the `start_node` and `target_node` in the graph. The graph is represented as a 
/// `HashMap` where each key is a node, and each value is a list of neighbors (outgoing edges).
/// The function returns an **optional vector** containing the path from `start_node` to `target_node` if a path exists, or `None` if no path is found.
///
/// # Arguments
/// * `graph` - The graph represented as a `HashMap<usize, Vec<usize>>` where each node points to a vector of neighbors.
/// * `start_node` - The node from which the search begins.
/// * `target_node` - The node to which the shortest path is being searched.
///
/// # Returns
/// Returns an `Option<Vec<usize>>`, where `Some(path)` is the shortest path if found, or `None` if no path exists.
pub fn bfs(graph: &HashMap<usize, Vec<usize>>, start_node: usize, target_node: usize) -> Option<Vec<usize>> {
    let mut visited = HashSet::new(); // Set to track visited nodes
    let mut queue = VecDeque::new(); // Queue for BFS to explore nodes level by level
    let mut parent = HashMap::new(); // Keeps track of the parent node to reconstruct the path

    queue.push_back(start_node); // Start BFS from the start node
    visited.insert(start_node); // Mark the start node as visited

    while let Some(node) = queue.pop_front() { // While there are nodes to explore
        if node == target_node { // If we reach the target node, reconstruct the path
            let mut path = Vec::new();
            let mut current = target_node;

            // Reconstruct the path from target node to start node
            while let Some(&p) = parent.get(&current) {
                path.push(current);
                current = p;
            }
            path.push(start_node);
            path.reverse(); // Reverse the path to get it from start to target
            return Some(path); // Return the reconstructed path
        }

        // Explore all neighbors of the current node
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) { // If neighbor has not been visited
                    visited.insert(neighbor); // Mark the neighbor as visited
                    parent.insert(neighbor, node); // Set the current node as the parent of the neighbor
                    queue.push_back(neighbor); // Add the neighbor to the queue for further exploration
                }
            }
        }
    }

    None // Return None if no path was found
}

/// Degree Centrality: Compute the degree centrality (in-degree and out-degree) for each node in the graph.
/// 
/// This function calculates the **degree centrality** for each node, which is the number of **incoming** and
/// **outgoing edges** (in-degree and out-degree) for each node in the graph. It prints the **top 10 nodes**
/// with the highest **out-degree**, which indicates the most connected nodes in terms of outgoing connections.
///
/// # Arguments
/// * `graph` - The graph represented as a `HashMap<usize, Vec<usize>>`, where each node points to a vector of neighbors (outgoing edges).
///
/// # Returns
/// This function does not return anything, but it prints the degree centrality (in-degree and out-degree)
/// for the top 10 nodes by out-degree.
pub fn degree_centrality(graph: &HashMap<usize, Vec<usize>>) {
    let mut in_degree: HashMap<usize, usize> = HashMap::new(); // Tracks in-degrees for each node
    let mut out_degree: HashMap<usize, usize> = HashMap::new(); // Tracks out-degrees for each node

    // Iterate through the graph and calculate in-degrees and out-degrees
    for (from_node, neighbors) in graph {
        *out_degree.entry(*from_node).or_insert(0) += neighbors.len(); // Add the number of outgoing edges to out-degree

        // For each neighbor, increment the in-degree for that neighbor
        for &to_node in neighbors {
            *in_degree.entry(to_node).or_insert(0) += 1; // Add an in-degree for each incoming edge
        }
    }

    // Sort nodes by out-degree in descending order
    let mut degree_list: Vec<_> = graph.keys().collect();
    degree_list.sort_by(|a, b| out_degree.get(b).unwrap_or(&0).cmp(&out_degree.get(a).unwrap_or(&0)));

    // Print the top 10 nodes by out-degree
    for node in degree_list.iter().take(10) {
        let in_d = in_degree.get(*node).cloned().unwrap_or(0); // Get in-degree, default to 0 if not found
        let out_d = out_degree.get(*node).cloned().unwrap_or(0); // Get out-degree, default to 0 if not found
        println!("Node {}: In-degree = {}, Out-degree = {}", node, in_d, out_d); // Print the degree centrality of the node
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // Test for BFS: Find the shortest path between two nodes.
    #[test]
    fn test_bfs() {
        let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
        graph.insert(1, vec![2]);
        graph.insert(2, vec![3]);
        graph.insert(3, vec![4]);
        graph.insert(4, vec![]);

        let path = bfs(&graph, 1, 4);
        assert_eq!(path, Some(vec![1, 2, 3, 4]));  // BFS should find the shortest path
    }

    // Test for Degree Centrality: Print the degree centrality of the top 10 nodes
    #[test]
    fn test_degree_centrality() {
        let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
        graph.insert(1, vec![2, 3]);
        graph.insert(2, vec![3]);
        graph.insert(3, vec![]);

        degree_centrality(&graph);  // We expect to see output for degree centrality
    }
}
