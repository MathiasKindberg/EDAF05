use std::io::{self, Read};

// Get-Content -Path .\1.in -Raw | cargo run
// cargo run < 1.in
// add --release for optimized version.

//#[derive(Debug, Clone)]


fn get_input() -> Result<Vec<String>, std::io::Error> {
    let mut raw_input = String::new();
    io::stdin().read_to_string(&mut raw_input)?;
    raw_input = raw_input.replace(&['\u{feff}', '\r'][..], ""); // Remove list of annoying characters.

    let mut clean_input: Vec<String> = raw_input
        .split(&[' ', '\n'][..]) // Split the input by line and/or space
        .map(|s| s.to_string()) // Convert the &str to string.
        .collect(); // Collect into vector.

    clean_input.retain(|s| s != ""); // Remove empty elements.
    Ok(clean_input)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Lab 3");
    Ok(())
}