// cytotoxicity and unreservedness have the same letter sum as each other (188), and they have no letters in common.
// Find a pair of words that have no letters in common, and that have the same letter sum, which is larger than 188.
// (There are two such pairs, and one word appears in both pairs.)

use std::{
    clone,
    cmp::Ordering,
    collections::{HashMap, HashSet},
    result,
};

use array_tool::vec::Intersect;

use super::challenge_2::assign_value;

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
            .unwrap_or(&"verdammt".to_string())
            .to_owned();

        // iter over each group of word
        for word in word_group.1.clone() {
            // when we found two words that satisfies all the conditions
            // we break both loops (the first by using a flag)
            if !letters_in_common(&tmp, &word as &str) {
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

// The Rusty way
pub fn words_diff_letters_v2(charmap: &HashMap<char, i32>) -> HashSet<(String, String)> {
    // using HashSet to avoid repeated pairs
    let mut result: HashSet<(String, String)> = HashSet::new();

    // iterate over the main HashMap
    let precached_list = assign_value(charmap);

    // get keys from 188
    let keys = precached_list
        .keys()
        .filter(|&&key| key >= 188)
        .collect::<Vec<_>>();

    // iterate over the groups of words
    for key in keys.iter() {
        let group = precached_list.get(key).unwrap().to_owned();
        // take each word and check if they have letters in common
        for word in group.iter() {
            let search = group
                .iter()
                .find(|&another_word| !letters_in_common(word, another_word));

            // match each case
            match search {
                Some(another_word) => {
                    // this way we filter the repeated pairs
                    let tupla = if word.cmp(another_word) == Ordering::Greater {
                        (word.clone(), another_word.clone())
                    } else {
                        (another_word.clone(), word.clone())
                    };

                    result.insert(tupla);
                }
                None => {}
            }
        }
    }

    result
}
/**
 * This function compares two words and returns true if they have any letter in common
 */
fn letters_in_common(first: &str, second: &str) -> bool {
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
