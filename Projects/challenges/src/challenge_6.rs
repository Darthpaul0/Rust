// cytotoxicity and unreservedness have the same letter sum as each other (188), and they have no letters in common.
// Find a pair of words that have no letters in common, and that have the same letter sum, which is larger than 188.
// (There are two such pairs, and one word appears in both pairs.)

use std::{clone, collections::HashMap, result};

use array_tool::vec::Intersect;

use crate::challenge_2::assign_value;

pub fn words_diff_letters(charmap: &HashMap<char, i32>) -> HashMap<i32, Vec<String>> {
    let mut result: HashMap<i32, Vec<String>> = HashMap::new();

    // iterate over the main HashMap
    let precached_list = assign_value(charmap);

    // aux variables
    let mut index = 0;
    let mut flag = false;

    // get group of words from each index from the original HashMap
    for word_group in precached_list {
        // store a temporal word to compare
        let tmp = word_group
            .1
            .get(index)
            .clone()
            .unwrap_or(&"nichts".to_string())
            .to_owned();
        println!("------");
        println!("{}", tmp);
        // iter over each group of word
        for word in word_group.1.clone() {
            println!("{}", word);
            println!("------");
            if !letters_in_common(tmp.clone(), word.clone()) {
                result.insert(word_group.0, vec![tmp, word]);
                flag = true;
                break;
            }
        }
        index += 1;
        if flag {
            break;
        }
    }

    result
}
/**
 * This function compares two words and returns true if they have any letter in common
 */
fn letters_in_common(first: String, second: String) -> bool {
    // turn words into vectors
    let vec_first = first.chars();
    let vec_second = second.chars();
    let mut flag = false;

    for letter in vec_first {
        for letra in vec_second.clone() {
            if letter == letra {
                flag = true;
                break;
            }
        }
    }

    flag
}
