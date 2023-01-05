// The list of word { geographically, eavesdropper, woodworker, oxymorons } contains 4 words.
// Each word in the list has both a different number of letters, and a different letter sum.
// The list is sorted both in descending order of word length, and ascending order of letter sum. What's the longest such list you can find?

use std::clone;
use std::collections::HashMap;

use crate::challenge_1::lettersum;
use crate::challenge_2::lines_from_file;

pub fn longest_list() -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let charmap: HashMap<char, i32> = HashMap::new();

    // get list of words from the original list
    let mut original_list = lines_from_file("./words_alpha.txt");

    // order these words by descending length and ascending lettersum
    original_list.sort_by(|a, b| b.len().partial_cmp(&a.len()).unwrap());

    // aux
    let mut last_word: String = String::from("pollosypavas".to_string());

    // find the longest list of words with different length and different lettersum
    for word in original_list.clone() {
        let tmp = original_list.get(0..original_list.len()).unwrap();
        for wort in tmp.iter() {
            if wort.len() < word.len()
                && wort.len() < last_word.len()
                && lettersum(&wort, &charmap) < lettersum(&word, &charmap)
            {
                result.push(wort.to_owned());
                last_word = result.pop().unwrap_or("sejodióelperú".to_string());
                println!("{:?}", result);
                break;
            }
        }
    }

    result
}
