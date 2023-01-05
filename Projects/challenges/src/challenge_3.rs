// Challenge 3: How many words have an odd letter sum?
// Function that return those words wich sum is an odd number
use std::collections::HashMap;

use crate::challenge_2::assign_value;

pub fn odd_words(charmap: &HashMap<char, i32>) -> i32 {
    // define word groups by sum
    let cached_words = assign_value(charmap);
    let mut many_words: i32 = 0;

    for (key, value) in cached_words {
        if key % 2 == 0 {
            many_words += value.len() as i32
        }
    }
    many_words
}
