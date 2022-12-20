use std::collections::HashMap;

fn main() {
    // Create new HasMap
    let mut charmap: HashMap<char, i32> = HashMap::new();

    // fill HashMap with letter and its value
    for (i, letter) in ('a'..='z').enumerate() {
        charmap.insert(letter, i as i32 + 1);
    }

    // some testing
    assert_eq!(lettersum(String::from("abcd"), &charmap), 10);
    assert_eq!(lettersum(String::from(""), &charmap), 0);
    assert_eq!(lettersum(String::from("a"), &charmap), 1);
    assert_eq!(lettersum(String::from("z"), &charmap), 26);
    assert_eq!(lettersum(String::from("cab"), &charmap), 6);
    assert_eq!(lettersum(String::from("excellent"), &charmap), 100);
    assert_eq!(
        lettersum(String::from("microspectrophotometries"), &charmap),
        317
    );

    println!("{}", lettersum(String::from("spain"), &charmap))
}

//Challenge 1: Assign every lowercase letter a value,
// from 1 for a to 26 for z.
fn lettersum(word: String, charmap: &HashMap<char, i32>) -> i32 {
    // convert word to iterator
    word.chars()
        // check if a letter is in the HashMap
        .map(|c| charmap.get(&c).unwrap_or(&0).to_owned())
        // add the result obtained
        .sum()
}
