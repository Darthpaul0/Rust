// cytotoxicity and unreservedness have the same letter sum as each other (188), and they have no letters in common.
// Find a pair of words that have no letters in common, and that have the same letter sum, which is larger than 188.
// (There are two such pairs, and one word appears in both pairs.)

use std::{collections::HashMap, result};

use array_tool::vec::Intersect;

use crate::challenge_2::assign_value;

pub fn words_diff_letters(charmap: &HashMap<char, i32>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    // iterate over the main HashMap
    let precached_list = assign_value(charmap);

    // get group of words from each index
    for word_group in precached_list {}

    // iterate over each word in the group comparing its characters

    result
}
/**
 * This function compares two words and returns true if they don't have any letter in common
 */
pub fn compare_words(first: String, second: String) -> bool {
    // trait Chars {}
    // // turn words into vectors
    // let vec_first = vec![first.chars()];
    // let vec_second = vec![second.chars()];

    // if vec_first.intersect(vec_second) {
    //     return false;
    // }
    true
}
