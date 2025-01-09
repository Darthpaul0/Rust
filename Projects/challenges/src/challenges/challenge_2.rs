// Challenge 2: Create a function that finds a word with a given sum
// in the word list provided. For instance, polytetrafluoroethylene is
// the only word with a sum of 313.

use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use super::challenge_1::lettersum;

pub fn find_word_sum(sum: i32, charmap: &HashMap<char, i32>) -> Vec<String> {
    // 1. Read file
    let word_list = lines_from_file("./words_alpha.txt");

    word_list
        // make it iterable; using into_iter to get an owned version of the iterator
        .into_iter()
        // filter those words which the sum is the specificied
        .filter(|word| lettersum(word, charmap) == sum)
        .collect()
}

pub fn find_word_sum_iterative(sum: i32, charmap: &HashMap<char, i32>) -> Vec<String> {
    // 1. Read file
    let word_list = lines_from_file("./words_alpha.txt");
    let mut list = Vec::new();

    for word in word_list.into_iter() {
        if lettersum(word.as_str(), charmap) == sum {
            list.push(word);
        }
    }

    list
}

/**
 * Function to read a file and return a Vec<String> with its content
 */
pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

// Challenge 2.1
/** Function to assign a value to each word contained in the word list document */
pub fn assign_value(charmap: &HashMap<char, i32>) -> HashMap<i32, Vec<String>> {
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

pub fn word_group_by_sum(sum: i32, charmap: &HashMap<char, i32>) -> Vec<String> {
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
pub fn find_word_sum_improved(sum: i32, charmap: &HashMap<char, i32>) -> Vec<String> {
    // 1. Read file
    let word_list = lines_from_file("./words_alpha.txt");
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
