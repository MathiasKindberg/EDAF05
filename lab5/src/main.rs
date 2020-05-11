use std::io;
use std::io::prelude::*;
// Get-Content -Path .\1.in -Raw | cargo run
// Get-Content -Path .\data\sample\1.in -Raw | cargo run
// cargo run < 1.in
// add --release for optimized version.

#[derive(Debug, Clone)]
struct Point {
    x: isize,
    y: isize,
}

fn get_input() -> Result<Vec<Vec<String>>, std::io::Error> {
    let mut clean_input = Vec::new();
    // Read input by line.
    for line in io::stdin().lock().lines() {
        let mut line: Vec<String> = line
            .expect("Error converting stdin to string") // The lines operator can fail, just panic explicitly.
            .replace(&['\u{feff}', '\r'][..], "") // Remove annoying characters
            .split(&[' '][..]) // Split the line baced on whitespace.
            .map(|s| s.to_string()) // Convert the &str to string.
            .collect(); // Collect into vector.

        line.retain(|s| s != ""); // Remove empty elements.

        // Make sure to not push empty lines, eg. often happens with the last line due to the input layout.
        if !line.is_empty() {
            clean_input.push(line);
        }
    }
    Ok(clean_input)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = get_input()?;

    // Just a bunch of array slicing and parsing to split up the input to the different parts and make them have the right type
    let chars = &input[0];
    let costs: Vec<Vec<isize>> = input[1..1 + chars.len()] // Slice the weight rows
        .iter() // Iterate over them
        .map(|row| {
            row.iter() // Take a row and Iterate over it
                .map(|s| s.parse::<isize>().expect("Error parsing input to integer")) // Parse into signed int
                .collect() // Collect into vec<isize>
        })
        .collect(); // Collect the Vec<isize> into Vec<Vec<isize>>

    let query_num: usize = input[chars.len() + 1][0].parse()?; // Positive number of queries, unsigned = good neough
    let queries = &input[chars.len() + 2..chars.len() + 2 + query_num];

    Ok(())
}
