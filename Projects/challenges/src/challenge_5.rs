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
    let word_list = precached_list.get_key_value(&sum).unwrap();

    for word in word_list.1 {
        let tmp_word = word_list.1.get(index).unwrap();
        println!("{} {:?}", word, tmp_word);

        if tmp_word.len() - word.len() == separation {
            separated_words.push(word.to_owned())
        }
        index += 1;
    }

    separated_words
}
