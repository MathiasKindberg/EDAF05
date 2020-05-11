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

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        // Need to explicitly convert from signed integer to f64 before doing sqrt.
        // All inputs are integers so the subtraction is valid.
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }
}

fn closest_point_distance(points: Vec<Point>) -> f64 {
    let n = points.len();
    let mut points_x = points;

    // Clone the points into the Y vector.
    let mut points_y = points_x.clone();

    // Sort the vectors. sort_unstable_by is faster than sort_by but does not keep ordering of
    // same size elements, which works for us.
    points_x.sort_unstable_by(|a, b| a.x.cmp(&b.x)); // O(n log n) worst case
    points_y.sort_unstable_by(|a, b| a.y.cmp(&b.y));
    closest(points_x, points_y, n)
}

fn closest(points_x: Vec<Point>, points_y: Vec<Point>, n: usize) -> f64 {
    // Brute force the distance of the last 3 points in the set. Usually like 40 according to lecture, too much
    // work setting up the datastructure to be worthwile for such a small set but 3 here to prove that the
    // implemented algorithm works for all cases.
    if n <= 3 {
        let mut min_distance = f64::MAX; // Essentially inf

        // Skip the last point since we compare second last to last either way.
        for i in 0..n - 1 {
            // All previous points has already been checked against our current, therefore we start from i + 1;
            for j in i + 1..n {
                min_distance = min_distance.min(points_x[i].distance(&points_x[j]));
            }
        }
        return min_distance;
    }

    // Due to ) and ] semantics in rust the right array will contain the pivot (n/2) and be longer in uneven
    // numbers, (+1) could be used to put in the left but it just adds noise.
    let l_x = points_x[0..n / 2].to_vec();
    let r_x = points_x[n / 2..].to_vec();

    let mut l_y = Vec::with_capacity(n / 2);
    let mut r_y = Vec::with_capacity(n / 2);

    let pivot = &points_x[n / 2];

    // Allocate y vectors by comparing the midpoint to have them on the right side.
    // Loop over p_y to preserve ordering. O(n)
    for point in &points_y {
        if point.x < pivot.x {
            // < because pivot is on right side
            l_y.push(Point {
                x: point.x,
                y: point.y,
            })
        } else {
            r_y.push(Point {
                x: point.x,
                y: point.y,
            });
        }
    }

    let l_n = l_x.len();
    let r_n = r_x.len();

    // Find the closest point by comparing the results of the two subproblems recursively until we hit the
    // brute force loop. This is how far from the pivot point we need to look at either side of the border.
    let mut delta = closest(l_x, l_y, l_n).min(closest(r_x, r_y, r_n));
    

    // Now find the elements we are interested in in O(n) time. In the book the pivot is in the left vector
    // due to the reasoning above here it is in the right. No real difference though.

    let mut within_delta = Vec::with_capacity(n); // n = long enough, happens once and better than resizing it every 2^n pushs.

    // Check which points are within the delta, add those to the check. O(n)
    for point in points_y {
        // Points are integers, delta is f64. Convert to f64.
        if ((point.x - pivot.x).abs() as f64) < delta {
            within_delta.push(point);
        }
    }

    let within_delta_len = within_delta.len();

    // Check all points in boundary, worst O(n) if they all are on a line.
    for i in 0..within_delta.len() {
        // Check the next 15 points, making sure to not index out of bounds.
        for j in i + 1..(i + 15).min(within_delta_len) {
            let distance = within_delta[i].distance(&within_delta[j]);
            if distance < delta {
                delta = distance;
            }
        }
    }
    delta
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
    let input = input[1..]
        .iter()
        .map(|s| s.parse::<isize>().expect("Error parsing input to integer"))
        .collect::<Vec<isize>>();

    let mut points = Vec::with_capacity(num_people);

    // Process input to points in chunks of 2.
    for point in input.chunks(2) {
        points.push(Point {
            x: point[0],
            y: point[1],
        })
    }

    // Calculate distance, display with 6 digits rounded.
    println!("{:.6}", closest_point_distance(points));
    // Algorithm takes about ~0.6s to run on huger dataset.

    Ok(())
}
