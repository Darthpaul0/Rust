use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    // Create new HasMap
    let mut char_map: HashMap<char, i32> = HashMap::new();

    // fill HashMap with letter and its value
    for (i, letter) in ('a'..='z').enumerate() {
        char_map.insert(letter, i as i32 + 1);
    }
    // CHALLENGES RESOLUTION
    // Challenge 1
    //println!("Challenge 1 result: {}",lettersum("microspectrophotometries", &charmap));

    // Challenge 2
    //println!("Challenge 2 result: {:?}", find_word_sum(313, &charmap));

    // Challenge 2.1
    println!(
        "Challenge 2.1 result: {:?}",
        word_group_by_sum(5, &char_map)
    );

    // Challenge 3
    //println!("Challenge 3 result: {}", odd_words(&charmap));

    // TESTING
    // some testing for challenge 1
    assert_eq!(lettersum("abcd", &char_map), 10);
    assert_eq!(lettersum("", &char_map), 0);
    assert_eq!(lettersum("a", &char_map), 1);
    assert_eq!(lettersum("z", &char_map), 26);
    assert_eq!(lettersum("cab", &char_map), 6);
    assert_eq!(lettersum("excellent", &char_map), 100);
    assert_eq!(lettersum("microspectrophotometries", &char_map), 317);

    // some testing for challenge 2
    dbg!("Original sum");
    let start = Instant::now();

    assert_eq!(find_word_sum(313, &char_map), ["polytetrafluoroethylene"]);
    assert_eq!(find_word_sum(1, &char_map), ["a"]);
    assert_eq!(find_word_sum(2, &char_map), ["aa", "b"]);

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    dbg!("Attempting to pre-cache every word in the dictionary");

    let cached_words = assign_value(&char_map);
    assert_eq!(cached_words.is_empty(), false);
    assert_eq!(
        cached_words
            .get(&313)
            .unwrap()
            .contains(&String::from("polytetrafluoroethylene")),
        true
    );

    dbg!("Calling 'improved' partial sum");

    let start = Instant::now();

    assert_eq!(
        find_word_sum_improved(313, &char_map),
        ["polytetrafluoroethylene"]
    );
    assert_eq!(find_word_sum_improved(1, &char_map), ["a"]);
    assert_eq!(find_word_sum_improved(2, &char_map), ["aa", "b"]);

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    dbg!("Calling iterative partial sum");

    let start = Instant::now();

    assert_eq!(
        find_word_sum_iterative(313, &char_map),
        ["polytetrafluoroethylene"]
    );
    assert_eq!(find_word_sum_iterative(1, &char_map), ["a"]);
    assert_eq!(find_word_sum_iterative(2, &char_map), ["aa", "b"]);

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    // some testing for challenge 3
}

//Challenge 1: Assign every lowercase letter a value,
// from 1 for a to 26 for z.
fn lettersum(word: &str, charmap: &HashMap<char, i32>) -> i32 {
    // convert word to iterator
    word.to_lowercase()
        .chars()
        // check if a letter is in the HashMap
        .map(|char| charmap.get(&char).unwrap_or(&0).to_owned())
        // add the result obtained
        .sum()
}

// Challenge 2: Create a function that finds a word with a given sum
// in the word list provided. For instance, polytetrafluoroethylene is
// the only word with a sum of 313.
fn find_word_sum(sum: i32, charmap: &HashMap<char, i32>) -> Vec<String> {
    // 1. Read file
    let word_list = lines_from_file("./words_alpha.txt");

    word_list
        // make it iterable; using into_iter to get an owned version of the iterator
        .into_iter()
        // filter those words which the sum is the specificied
        .filter(|word| lettersum(word, charmap) == sum)
        .collect()
}

fn find_word_sum_iterative(sum: i32, charmap: &HashMap<char, i32>) -> Vec<String> {
    // 1. Read file
    let word_list = lines_from_file("./words_alpha.txt");
    let mut list = Vec::new();

    for word in word_list.into_iter() {
        if lettersum(word.as_str(), &charmap) == sum {
            list.push(word);
        }
    }

    list
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

// Challenge 2.1
/** Function to assign a value to each word contained in the word list document */
fn assign_value(charmap: &HashMap<char, i32>) -> HashMap<i32, Vec<String>> {
    let word_list = lines_from_file("./words_alpha.txt");
    let mut word_map: HashMap<i32, Vec<String>> = HashMap::new();
    // fill HashMap with word and its value

    for word in word_list.into_iter() {
        let word_value = lettersum(&word, charmap);

        if word_map.contains_key(&word_value) {
            word_map.get_mut(&word_value).unwrap().push(word);
        } else {
            word_map.insert(word_value, vec![word]);
        }
    }

    // return Hashmap
    word_map
}

fn word_group_by_sum(sum: i32, charmap: &HashMap<char, i32>) -> Vec<String> {
    let word_map_sum = assign_value(charmap);
    let mut correct_words: Vec<String> = Vec::new();
    for (key, value) in word_map_sum.into_iter() {
        if key == sum {
            correct_words = value;
        }
    }
    correct_words
}

// Challenge 2.2
/**
 * It's improved because it will skip those words which sum is greater
 * than the specified sum before reading all chars
 */
fn find_word_sum_improved(sum: i32, charmap: &HashMap<char, i32>) -> Vec<String> {
    // 1. Read file
    let word_list: Vec<String> = lines_from_file("./words_alpha.txt");
    let mut correct_words: Vec<String> = Vec::new();

    for word in word_list.into_iter() {
        let mut tmp_sum = 0;

        for i in 0..word.len() {
            tmp_sum += lettersum(&word[i..=i], charmap);

            if tmp_sum > sum {
                break;
            }
        }

        if tmp_sum == sum {
            correct_words.push(word);
        }
    }

    correct_words
}

// Challenge 3: How many words have an odd letter sum?

//Function that return those words wich sum is an odd number

//fn odd_words(charmap: &HashMap<char, i32>) -> i32 {
//let words_list = lines_from_file("./words_alpha.txt");
//words_list.into_iter().filter(|odd_word| lettersum(odd_word, charmap) % 2 == 0).collect().
//}
