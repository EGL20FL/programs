use std::fs;
use std::io::{self, BufRead, Write};

fn remove_comments(input_file: &str) -> io::Result<()> {
    // Open the input file
    let file = fs::File::open(input_file)?;
    let reader = io::BufReader::new(file);

    // Create a temporary file
    let temp_file_path = format!("{}.tmp", input_file);
    let mut temp_file = fs::File::create(&temp_file_path)?;

    // Iterate through the lines of the input file
    for line in reader.lines() {
        let line = line?;
        // Remove comments
        let cleaned_line = line.split("//").next().unwrap_or("").trim(); // Remove whitespace

        // Check if the line is not empty
        if !cleaned_line.is_empty() {
            // Write the line without comments and not empty to the temporary file
            writeln!(temp_file, "{}", cleaned_line)?;
        }
    }

    // Replace the original file with the temporary file
    fs::rename(temp_file_path, input_file)?;

    Ok(())
}

fn main() -> io::Result<()> {
    // Ask the user for the input file path
    println!("Enter the file path ");

    // Read a line from standard input
    let mut input_filename = String::new();
    io::stdin().read_line(&mut input_filename)?;

    // Remove the newline character at the end of the string
    let input_filename = input_filename.trim();

    remove_comments(input_filename)?;

    println!("Comments removed. Result saved in {}", input_filename);
    Ok(())
}
