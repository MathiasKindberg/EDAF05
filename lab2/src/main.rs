use std::io::{self, Read};
// Get-Content -Path .\1.in -Raw | cargo run

#[derive(Debug, Clone)]
struct Word {
    word: String,
    index: usize,
    edges: Vec<usize>,
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

fn check_alike(word1: &str, word2: &str) -> bool {
    let mut used_chars = vec![0; 5]; // Keep track of which chars we have already matched on.

    // .chars() iterates over the string using Unicode Scalar Values, which works for English and similar
    // languages which only contain one scalar value per character. This would, as an example, not work on
    // emojis or asian characters which requires iteration and slicing over grapheme clusters instead.
    for ch1 in word1[1..].chars() {
        for (i, ch2) in word2.chars().enumerate() {
            if ch1 == ch2 && used_chars[i] == 0 {
                used_chars[i] = 1;
                break; // Don't match twice on the same character.
            }
        }
    }
    used_chars.iter().sum::<u32>() == 4
}

fn get_index(graph: &[Word], word: &str) -> Option<usize> {
    for v in graph {
        if v.word == word {
            return Some(v.index);
        }
    }
    None
}

fn bfs(graph: &[Word], start: usize, terminate: usize) -> String {
    // Initialize values we use while working the graph. Better here than
    // storing something unrelated to the graph in the node since these are
    // based on the search and not the actual structure.

    let mut visited: Vec<bool> = vec![false; graph.len()];
    visited[start] = true;

    let mut pred: Vec<Option<usize>> = vec![None; graph.len()];

    let mut q: Vec<usize> = Vec::new();
    q.push(start);

    if start == terminate {
        return "0".to_string();
    }

    while !q.is_empty() {
        let v = q.remove(0);

        for w in &graph[v].edges.clone() {
            if !visited[*w] {
                // * dereferences the reference.

                visited[*w] = true;
                pred[*w] = Some(v);

                q.push(*w);

                if *w == terminate {
                    let mut nbr_edges = 0;
                    let mut index = *w;

                    // Instead of the linked list like approach, we step through the array based
                    // on the index. I.e. doing the same but calling the index our pointer.
                    while index != start {
                        index = pred[index].unwrap();
                        nbr_edges += 1;
                    }
                    return (nbr_edges).to_string();
                }
            }
        }
    }
    "Impossible".to_string()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = get_input()?;
    let num_words: usize = input[0].parse()?;
    //let num_queries: usize = input[1].parse()?;

    // We store everything in a big vector which we index into since Rust is extremely picky about
    // cyclical data structures due to it's concurrency and memory correctness guarantees.
    let mut graph: Vec<Word> = Vec::new();

    // Index words and find edges.
    for (index, word) in input[2..num_words + 2].iter().enumerate() {
        let mut edges = Vec::new();
        for alike in &mut graph {
            if check_alike(word, &alike.word) {
                edges.push(alike.index);
            }
            // Checking backwards allows us to both index and build the graph in one go.
            if check_alike(&alike.word, word) {
                alike.edges.push(index);
            }
        }
        graph.push(Word {
            word: word.to_string(),
            index,
            edges,
        });
    }

    let mut output = String::new();

    // Process queries in chunks of 2.
    for query in input[num_words + 2..].chunks(2) {
        let start = get_index(&graph, &query[0]).expect("Word not found");
        let terminate = get_index(&graph, &query[1]).expect("Word not found");
        //println!("{}",bfs(&mut graph, start, terminate));
        output.push_str(&bfs(&graph, start, terminate));
        output.push('\n');
    }
    print!("{}", output);

    Ok(())
}
