use std::io::{self, Read};

// Get-Content -Path .\1.in -Raw | cargo run
// Get-Content -Path .\data\sample\1.in -Raw | cargo run
// cargo run < 1.in
// add --release for optimized version.

#[derive(Debug, Clone)]
struct Point {
    x: isize,
    y: isize,
}

fn closest(points_x: Vec<Point>, points_y: Vec<Point>, n: usize) -> isize {
    // Split arrays
    let l_x = points_x[0..n/2].to_vec();
    let r_x = points_x[n/2..].to_vec();

    let l_y = points_y[0..n/2].to_vec();
    let r_y = points_y[n/2..].to_vec();

    //closest(l_x, l_y, n/2);

    return 0;
}


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
    let input = get_input()?;
    println!("{:?}", input);
    let num_people: usize = input[0].parse()?;
    let input = input[1..]
        .iter()
        .map(|s| s.parse::<isize>().expect("Error parsing input to integer"))
        .collect::<Vec<isize>>();

    // Initialize point vector for x
    let mut points_x = Vec::new();

    for point in input.chunks(2) {
        points_x.push(Point{ x: point[0], y: point[1]})
    }
    // Copy the points into the Y vector.
    let mut points_y = points_x.clone();

    // Sort the vectors. sort_unstable_by is faster than sort_by but does not keep ordering of same size elements.
    points_x.sort_unstable_by(|a, b| a.x.cmp(&b.x)); // O(n log n) worst case
    points_y.sort_unstable_by(|a, b| a.y.cmp(&b.y));


    println!("x: {:?}", points_x);
    println!("y: {:?}", points_y);

    println!("{}", closest(&points_x, &points_y, num_people));
    
    Ok(())
}
