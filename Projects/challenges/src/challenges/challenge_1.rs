// Challenge 1: Assign every lowercase letter a value,
// from 1 for a to 26 for z.

use std::collections::HashMap;

pub fn lettersum(word: &str, charmap: &HashMap<char, i32>) -> i32 {
    // convert word to iterator
    word.to_lowercase()
        .chars()
        // check if a letter is in the HashMap
        .map(|char| charmap.get(&char).unwrap_or(&0).to_owned())
        // add the result obtained
        .sum()
}
