// CHALLENGE 5
// zyzzyva and biodegradabilities have the same letter sum as each other (151), and their lengths differ by 11 letters.
// Find the other pair of words with the same letter sum whose lengths differ by 11 letters.

use std::collections::HashMap;

use crate::challenge_2::assign_value;

pub fn words_equals_separated(
    sum: i32,
    separation: usize,
    charmap: &HashMap<char, i32>,
) -> Vec<String> {
    // declare new Vec
    let mut separated_words: Vec<String> = Vec::new();

    // define the pre-cached word list of words
    let precached_list = assign_value(charmap);

    // aux variables
    let mut index = 0;
    let error = (&8 as &i32, &vec![String::from("error")]);
    let no_word = &String::from("nothing");

    // get those words with the specified lettersum
    let words = precached_list.get_key_value(&sum).unwrap_or(error).1;
    let mut flag = true;

    while flag {
        // auxiliary variable to store the word to compare
        let tmp_word = words.get(index).unwrap_or(no_word);

        // compare aux variable with each word
        for word in words {
            // we break in the first occurrence
            if word.len().abs_diff(tmp_word.len()) == separation {
                // push both words to show later
                separated_words.push(tmp_word.to_owned());
                separated_words.push(word.to_owned());
                break;
            }
            // break loop
            flag = false;
        }
        index += 1;
    }
    // provide some information to user
    if separated_words.is_empty() {
        separated_words.push(String::from("No words found!"));
        separated_words
    } else {
        separated_words
    }
}
