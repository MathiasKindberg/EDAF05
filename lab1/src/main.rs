use std::io::{self, Read};

#[derive(Debug, Clone)]
struct Person {
    index: usize,
    pref_list: Vec<usize>,
}

// PS cmd to start. --release for optimized version.
// Get-Content -Path .\2testmid.in -Raw | cargo run

// Gets the input from Stdin and then splits it into a vector with a number
// in each field. 
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

fn parse_input(input: Vec<String>) -> Result<(Vec<Person>, Vec<Person>), Box<dyn std::error::Error>> {
    let num_people: usize = input[0].parse()?;

    // Computation is done on indexes, create vectors of right length containing none values.
    let mut men = vec![None; num_people];
    let mut women = vec![None; num_people];

    // Iterate over the input in chunks, num_people + 1 because each line is the person index and then their
    // 4 preferences. Starting from 1 because the first item is the number of each gender.
    for chunk in input[1..].chunks(num_people + 1) {
        // Parse the chunk containing string values into integers.
        let input = chunk
            .iter   ()
            .map(|s| {
                s.parse::<usize>()
                    .expect("Error parsing person input to integer")
                    - 1 // - 1 since we go to 0 based indexing.
            })
            .collect::<Vec<usize>>();

        let index = input[0];
        // As given by the instructions, the first occurrence is a woman, second man.
        if women[index].is_none() {
            // Create empty inverted list to populate
            let mut inv_list = vec![0; num_people];

            // Iterate over and invert the input to reduce lookups in GS.
            for (i, x) in input[1..].to_vec().into_iter().enumerate() {
                inv_list[x] = i;
            }

            women[index] = Some(Person {
                index,
                pref_list: inv_list,
            });
        } else {
            men[index] = Some(Person {
                index,
                pref_list: input[1..].to_vec(),
            });
        }
    }

    // We now have complete cleaned lists without none values, unwrap them into nice usable 
    // constant vectors. Filter_map simply removes all none values. 
    let women: Vec<Person> = women.into_iter().filter_map(|x| x).collect();
    let men: Vec<Person> = men.into_iter().filter_map(|x| x).collect();
    Ok((women, men))
}


fn gs(women: Vec<Person>, men: Vec<Person>) -> Option<Vec<Option<usize>>> {
    // Array index = women, integer stored = man. None values to show unassigned.
    let mut pairs = vec![None; women.len()];
    let mut p = men.clone(); // Create list

    // Iterate over the list until empty.
    while let Some(m) = p.pop() {
        let w = &women[m.pref_list[0]]; // The woman the man prefers.

        if pairs[w.index] == None {
            pairs[w.index] = Some(m.index);
        } else if w.pref_list[m.index] < w.pref_list[pairs[w.index]?] {
            let to_remove = &men[pairs[w.index]?];

            pairs[w.index] = Some(m.index);
            p.push(Person {
                index: to_remove.index,
                pref_list: to_remove.pref_list[1..].to_vec(), // Set the next pairing to 0 on pref_list.
            });
        } else {
            p.push(Person {
                index: m.index,
                pref_list: m.pref_list[1..].to_vec(), // Set the next pairing to 0 on pref_list.
            });
        }
    }
    Some(pairs)
}

// Just box up any error that is encountered, allows us to us ? on any type that somehow implements the std error type.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = get_input()?;
    let (women, men) = parse_input(input)?;

    // Run and print output.
    gs(women, men)
        .expect("Error when running GS algorithm")
        .iter()
        .for_each(|x| println!("{}", x.unwrap() + 1));

    Ok(())
}