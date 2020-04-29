use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, Read};

// Get-Content -Path .\1.in -Raw | cargo run
// Get-Content -Path .data\sample\1.in -Raw | cargo run
// cargo run < 1.in
// add --release for optimized version.

#[derive(Debug, Clone)]
struct Edge {
    node: usize,
    weight: usize,
}

// Essentially pointless but makes the code look nicer, could use a Vec<Vec<Edge>> instead.
// Would have a purpose if the nodes contained some other information than their edges.
#[derive(Debug, Clone)]
struct Node {
    edges: Vec<Edge>,
}

// The state of the program, essentially an edge with a weight added to the queue which is then
// popped based on the lowest cost.
#[derive(Copy, Clone, Eq)]
struct State {
    cost: usize,
    index: usize,
}

// Required for sorting.
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

// Required for sorting.
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // Flip order of the comparison (other.cost first, instead of self.cost).
        // this makes the heap a min heap instead of max heap which is the default.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.index.cmp(&other.index))
    }
}

// Required for sorting.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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

fn prim(graph: Vec<Node>) -> usize {
    let mut cost = 0;
    let mut checked = vec![false; graph.len()];
    let mut heap = BinaryHeap::new(); // Priority queue.

    // "Randomly" choose the first node to be our root.
    heap.push(State { cost: 0, index: 0 });

    // Iterate until the heap is empty
    while let Some(state) = heap.pop() {
        if !checked[state.index] {
            cost += state.cost;

            // Add the neighbours to the queue.
            // O(n) (m = number of nodes)
            for edge in &graph[state.index].edges {
                // Node u -> v, if v is not explored. Add to queue.
                if !checked[edge.node] {
                    heap.push(State {
                        cost: edge.weight,
                        index: edge.node,
                    }); // O(log(m)) (m = number of edges)
                }
            }
            checked[state.index] = true;
        }
    }
    cost
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = get_input()?;
    let num_people: usize = input[0].parse()?;

    let input = input[2..]
        .iter()
        .map(|s| s.parse::<usize>().expect("Error parsing input to integer"))
        .collect::<Vec<usize>>();
    // Initialize the empty graph.
    let mut graph: Vec<_> = (0..num_people)
        .map(|_| Node { edges: Vec::new() })
        .collect();

    // Build the graph
    for edge in input.chunks(3) {
        // Bidirectional. Sprinkles of -1 for zero based positional indexing.
        graph[edge[0] - 1].edges.push(Edge {
            node: edge[1] - 1,
            weight: edge[2],
        });
        graph[edge[1] - 1].edges.push(Edge {
            node: edge[0] - 1,
            weight: edge[2],
        });
    }
    // Getting the input and building the graph takes ~2.5 seconds on huge dataset.

    println!("{}", prim(graph));
    // The prim algorithm takes ~1.2 seconds on huge dataset.

    Ok(())
}
