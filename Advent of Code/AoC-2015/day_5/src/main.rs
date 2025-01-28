use std::{
    collections::{HashMap, HashSet},
    fs,
    hash::Hash,
};

fn main() {
    println!(
        "There are {} nice strings",
        classify_strings("./src/strings.txt".to_string())
    );
}

fn filter_string(input: &str) -> bool {
    // Check first condition: At least three vowels
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let disallowed = ["ab", "cd", "pq", "xy"];

    let vowels_count = input.chars().filter(|&c| vowels.contains(&c)).count();
    let vowels_condition = vowels_count >= 3;

    // Check second condition: At least one repeated letter
    let repeated_condition = input
        .chars()
        .zip(input.chars().skip(1))
        .any(|(c1, c2)| c1 == c2);

    // Check third condition: No disallowed substrings
    let excluded_condition = !disallowed.iter().any(|&sub| input.contains(sub));

    vowels_condition && repeated_condition && excluded_condition
}

fn second_filtering(input: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();
    let mut pair_found = false;
    let mut repeat_with_gap_found = false;

    // Check for a pair that appears at least twice without overlapping
    for i in 0..chars.len() - 1 {
        let pair = &input[i..i + 2];
        if input[i + 2..].contains(pair) {
            pair_found = true;
            break;
        }
    }

    // Check for a letter that repeats with exactly one letter between them
    for i in 0..chars.len() - 2 {
        if chars[i] == chars[i + 2] {
            repeat_with_gap_found = true;
            break;
        }
    }

    pair_found && repeat_with_gap_found
}

fn classify_strings(path: String) -> usize {
    let strings_list = extract_strings(path);
    strings_list
        .iter()
        .filter(|word| second_filtering(word))
        .count()
}

fn extract_strings(path: String) -> Vec<String> {
    let content = fs::read_to_string(path).expect("Failed to read file");
    content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().to_string())
        .collect()
}
