use std::collections::HashMap;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    // Create new HasMap
    let mut charmap: HashMap<char, i32> = HashMap::new();

    // fill HashMap with letter and its value
    for (i, letter) in ('a'..='z').enumerate() {
        charmap.insert(letter, i as i32 + 1);
    }
    // CHALLENGES RESOLUTION
    // Challenge 1
    println!(
        "This is challenge 1 result: {}",
        lettersum(String::from("microspectrophotometries"), &charmap)
    );

    // Challenge 2
    println!("Challenge 2 result: {:?}", find_word_sum(313, &charmap));

    // Challenge 3
    println!("Challenge 3 result: {}", odd_words(&charmap));

    // TESTING
    // some testing for challenge 1
    assert_eq!(lettersum(String::from("abcd"), &charmap), 10);
    assert_eq!(lettersum(String::from(""), &charmap), 0);
    assert_eq!(lettersum(String::from("a"), &charmap), 1);
    assert_eq!(lettersum(String::from("z"), &charmap), 26);
    assert_eq!(lettersum(String::from("cab"), &charmap), 6);
    assert_eq!(lettersum(String::from("excellent"), &charmap), 100);
    assert_eq!(
        lettersum(String::from("microspectrophotometries"), &charmap),
        317
    );

    // some testing for challenge 2
    assert_eq!(find_word_sum(313, &charmap), ["polytetrafluoroethylene"]);
    assert_eq!(find_word_sum(1, &charmap), ["a"]);
    assert_eq!(find_word_sum(2, &charmap), ["aa", "b"])
}

//Challenge 1: Assign every lowercase letter a value,
// from 1 for a to 26 for z.
fn lettersum(word: String, charmap: &HashMap<char, i32>) -> i32 {
    // convert word to iterator
    word.to_lowercase()
        .chars()
        // check if a letter is in the HashMap
        .map(|c| charmap.get(&c).unwrap_or(&0).to_owned())
        // add the result obtained
        .sum()
}

// Challenge 2: Create a function that finds a word word with a given sum
// in the word list provided. For instance, polytetrafluoroethylene is
// the only word with a sum of 313.
fn find_word_sum(sum: i32, charmap: &HashMap<char, i32>) -> Vec<String> {
    // 1. Read file
    let word_list = lines_from_file("./words_alpha.txt");
    let mut correct_words: Vec<String> = Vec::new();

    // 2. Compare the sum of each word with the provided sum
    for word in word_list {
        if lettersum(word.to_owned(), charmap) == sum {
            // 3. Return the words that satisfy the condition
            correct_words.push(word);
        }
    }
    correct_words
}

/**
 * Function to read a file and return a Vec<String> with its content
 */
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

// Challenge 3: How many words have an odd letter sum?
/**
 * Function that return those words wich sum is an odd number
 */
fn odd_words(charmap: &HashMap<char, i32>) -> i32 {
    let words_list = lines_from_file("./words_alpha.txt");
    let mut odd_words: i32 = 0;
    for word in words_list {
        if lettersum(word.to_owned(), charmap) % 2 != 0 {
            odd_words = odd_words + 1;
        }
    }
    odd_words
}
