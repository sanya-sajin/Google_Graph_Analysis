use std::collections::HashMap;
use csv::ReaderBuilder;

/// Builds a graph from the CSV file and returns it as a HashMap.
/// Each node points to a vector of its neighbors (outgoing edges).
///
/// # Arguments
/// * `file_path` - Path to the CSV file containing the graph's edge list.
///
/// # Returns
/// This function returns a `Result<HashMap<usize, Vec<usize>>, Box<dyn std::error::Error>>` representing the graph as a HashMap where the key is the node ID and the value is a
/// vector of neighbors.
pub fn build_graph_from_csv(file_path: &str) -> Result<HashMap<usize, Vec<usize>>, Box<dyn std::error::Error>> {
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(file_path)?;

    // Parse the CSV file and construct the graph
    for result in rdr.records() {
        let record = result?; // Parse each CSV record
        let from_node: usize = record[0].parse()?; // FromNodeId
        let to_node: usize = record[1].parse()?; // ToNodeId

        // Insert the edge into the graph using entry to avoid panic
        graph.entry(from_node).or_insert_with(Vec::new).push(to_node);
        
        // Ensure the `to_node` is inserted into the graph as well (even if it has no outgoing edges)
        graph.entry(to_node).or_insert_with(Vec::new);
    }

    Ok(graph)
}

