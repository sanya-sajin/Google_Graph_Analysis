use std::fs::File;
use std::io::{self, BufRead};
use csv::Writer;

/// Converts a TXT file (web-Google.txt) to a CSV file (web-Google.csv).
/// This function reads a TXT file where each line represents an edge and writes it to CSV.
///
/// # Arguments
/// * `input_file` - The path to the input TXT file containing graph edges.
/// * `output_file` - The path to the output CSV file to write to.
///
/// # Returns
/// This function returns a `Result<(), io::Error>` indicating success or failure.
pub fn convert_txt_to_csv(input_file: &str, output_file: &str) -> io::Result<()> {
    let file = File::open(input_file)?; // Open the input file
    let reader = io::BufReader::new(file); // Efficient buffered reading

    let mut wtr = Writer::from_path(output_file)?; // Create a CSV writer

    // Write header for CSV
    wtr.write_record(&["FromNodeId", "ToNodeId"])?;

    // Read each line of the TXT file
    for line in reader.lines() {
        let line = line?; // Get each line
        if line.starts_with("#") {
            continue; // Skip comment lines
        }

        // Split line into two parts (FromNodeId and ToNodeId)
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let from_node = parts[0];
            let to_node = parts[1];

            // Write to CSV file
            wtr.write_record(&[from_node, to_node])?;
        }
    }

    wtr.flush()?; // Ensure all data is written to the file
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    // Test the CSV conversion function
    #[test]
    fn test_convert_txt_to_csv() {
        let input_file = "test_input.txt";
        let output_file = "test_output.csv";

        // Create a mock input file with some graph data
        let _ = fs::write(input_file, "# Sample graph\n1 2\n2 3\n");

        // Run the conversion function
        let result = convert_txt_to_csv(input_file, output_file);

        // Check that the conversion succeeded
        assert!(result.is_ok());

        // Read the output CSV file and check its contents
        let contents = fs::read_to_string(output_file).expect("Unable to read file");
        assert!(contents.contains("1,2"));
        assert!(contents.contains("2,3"));

        // Clean up test files
        fs::remove_file(input_file).expect("Unable to delete test input file");
        fs::remove_file(output_file).expect("Unable to delete test output file");
    }
}
