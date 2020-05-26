#![allow(unused_variables)]

use std::collections::HashMap;
use std::io;


fn main() {
    // Intro
    println!(
        "\nThe String Cutter v0.1.0 cuts your string and returns the frequency of the search element."
    );
    println!("created by DracoY\n");

    // The Map
    let mut map: HashMap<String, u32> = HashMap::new();

    // Error Handle
    let error = String::from("Failed to read from stdin!");

    // Inputs
    println!("Please enter a string.");

    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect(&error);
    let line: String = line.trim().to_string();
    
    println!("Please enter the search element.");
    
    let mut search = String::new();
    io::stdin().read_line(&mut search)
        .expect(&error);
    let search: String = search.trim().to_string();

    // Creating Map Data
    for word in line.trim().split_whitespace() {
        let count = map.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    // Accessing the search element from the map data
    let frequency = map.get(&search);

    // Handling Option<&u32>
    let frequency: u32 = match frequency {
        Some(frequency) => *frequency,
        None => 0
    };

    // Printing data to be displayed to the user
    println!("\nThe frequency of {} is {:?}", search, frequency);

    // Outro
    println!("Thanks for using! Have a great day!");

    // References
    println!("\nFor reference: \n{:?}", map);
}
