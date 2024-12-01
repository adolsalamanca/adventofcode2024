use std::fs::File;
use std::io::{self, BufRead, Cursor};
use std::path::Path;

/// Reads lines from a file and processes them into two sorted arrays of `u32`.
pub(crate) fn read_vectors_from_file<P>(filename: P) -> Result<(Vec<u32>, Vec<u32>), io::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    lines_sorted(reader)
}

/// Reads lines from a string and processes them into two sorted arrays of `u32`.
#[allow(dead_code)]
pub(crate) fn read_vectors_from_str(input: &str) -> Result<(Vec<u32>, Vec<u32>), io::Error> {
    let reader = Cursor::new(input.as_bytes()); // Convert string to a cursor of bytes
    lines_sorted(reader)
}

/// Processes lines from a `BufRead` and splits them into two sorted `Vec<u32>` arrays.
fn lines_sorted(input: impl BufRead) -> Result<(Vec<u32>, Vec<u32>), io::Error> {
    let mut from_arr = Vec::new();
    let mut to_arr = Vec::new();

    // Iterate through the lines in the input
    for line in input.lines() {
        match line {
            Ok(line) => {
                let parts: Vec<&str> = line.split_whitespace().collect(); // Split by whitespace

                for part in parts {
                    if part.is_empty() {
                        continue; // Skip empty parts
                    }

                    // Parse the number and push it to the appropriate vector
                    let number = part.parse::<u32>().map_err(|e| {
                        println!("number to parse: {}" ,part);
                        io::Error::new(io::ErrorKind::InvalidData, format!("Parse error: {}", e))
                    })?;

                    if from_arr.len() == to_arr.len() {
                        from_arr.push(number);
                    } else {
                        to_arr.push(number);
                    }
                }
            }
            Err(e) => return Err(e),
        }
    }

    from_arr.sort();
    to_arr.sort();

    Ok((from_arr, to_arr))
}
