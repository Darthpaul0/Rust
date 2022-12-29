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
    let mut index = 0;

    // search in the precached list words separated by the specified length
    // with the specified sum
    let error = (&8 as &i32, &vec![String::from("error")]);
    let no_word = &String::from("nothing");
    let words = precached_list.get_key_value(&sum).unwrap_or(error).1;
    let mut flag = true;

    while flag {
        let tmp_word = words.get(index).unwrap_or(no_word);
        for word in words {
            if word.len().abs_diff(tmp_word.len()) == separation {
                separated_words.push(tmp_word.to_owned());
                separated_words.push(word.to_owned());
                break;
            }
            flag = false;
        }
        index += 1;
    }
    if separated_words.len() == 0 {
        separated_words.push(String::from("No words found!"));
        separated_words
    } else {
        separated_words
    }
}
