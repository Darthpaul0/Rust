// Challenge 4
// There are 1921 words with a letter sum of 100, making it the second most common letter sum.
// What letter sum is most common, and how many words have it?

use std::collections::HashMap;

use crate::challenge_2::assign_value;

pub fn most_common_sum(charmap: &HashMap<char, i32>) -> Vec<String> {
    // define word groups by sum
    let cached_words = assign_value(charmap);
    let most_common_sum: Vec<String>;

    // compare values and assign the most common group
    most_common_sum = cached_words
        // get HashMap keys
        .into_values()
        // get the length of every group of words
        .max_by(|x, y| x.len().cmp(&y.len()))
        .unwrap();
    most_common_sum
}
