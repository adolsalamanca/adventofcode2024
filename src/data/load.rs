use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

#[allow(dead_code)]
pub(crate) fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



/// A function to read lines from a file and transform them using a closure.
///
/// `transform` is a closure that takes a `String` (the line) and returns a value of type `T`.
/// The function returns a `Result<Vec<T>, io::Error>`, where `Vec<T>` contains the transformed lines.
#[allow(dead_code)]
pub(crate) fn read_lines_with_transform<P, F, T>(
    filename: P,
    transform: &F,  // Pass `transform` by reference
) -> io::Result<Vec<T>>
where
    P: AsRef<Path>,
    F: Fn(String) -> T,  // Closure type: takes a `String`, returns `T`
{
    // Open the file
    let file = File::open(filename)?;

    // Create a BufReader and iterate over the lines
    let reader = io::BufReader::new(file);

    // Map each line using the `transform` closure and collect the results into a vector
    reader
        .lines() // Returns an iterator over the lines
        .map(|line| line.map(transform)) // Apply `transform` to each line
        .collect() // Collect results into a vector
}