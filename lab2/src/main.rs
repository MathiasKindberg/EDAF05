use std::io::{self, Read};
// Get-Content -Path .\1.in -Raw | cargo run


// Either one big vector or use hashmap for storage.
//  https://doc.rust-lang.org/std/collections/struct.HashMap.html
struct Word {
    word: String,
    index: usize,
    connections: Vec<usize>,
}


fn get_input() -> Result<Vec<String>, std::io::Error> {
    let mut raw_input = String::new();
    io::stdin().read_to_string(&mut raw_input)?;
    raw_input = raw_input.replace(&['\u{feff}', '\r'][..], ""); // Remove list of annoying characters.

    let mut clean_input: Vec<String> = raw_input
        .split(&[' ', '\n'][..]) // Split the input by line and/or space
        .map(|s| s.to_string()) // Convert the &str to string.
        .collect();             // Collect into vector.

    clean_input.retain(|s| s != ""); // Remove empty elements.
    Ok(clean_input)
}

fn check_alike(word1: &String, word2: &String) -> bool {
    let mut used_chars = vec![0; 5]; // Keep track of which chars we have already matched on.

    for ch1 in word1[1..].chars() {
        for (i, ch) in word2.chars().enumerate() {
            if ch == ch1 && used_chars[i] == 0 {
                used_chars[i] = 1;
                break; // Don't match twice on the same character.
            }
        }
    }
    used_chars.iter().sum::<u32>() == 4
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = get_input()?;
    let num_words:usize = input[0].parse()?;
    let num_connections:usize = input[1].parse()?;

    println!("there retch: {}", check_alike(&"there".to_string(), &"retch".to_string()));
    println!("where there: {}", check_alike(&"where".to_string(), &"there".to_string()));
    println!("hello lolem: {}", check_alike(&"hello".to_string(), &"lolem".to_string()));
    println!("lolem hello: {}", check_alike(&"lolem".to_string(), &"hello".to_string()));

    println!("{:?}", num_words);
    println!("{:?}", num_connections);

    let mut words = Vec::new();

    for word in input[2..num_words+2].iter() {
        println!("{}", word);
        for alike in &words {
            if check_alike(word, alike) {
                // Words alike, do magic.
            }
            if check_alike(alike, word) {
                // Words alike, do magic
            }

            // Remember, we need to check both ways, since some words have double connections
            // and others dont.
            println!("{} {}: {}", word, alike, check_alike(word, alike));
            println!("{} {}: {}", alike, word, check_alike(alike, word));

        }
        words.push(word.to_string());
    }

    // Process connection queries in chunks of 2.
    for connection in input[num_words + 2..].chunks(2) {
        println!("{:?}", connection);
    }


    Ok(())
}

