use std::{collections::HashMap, fs};

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
    let mut repeated_between = false;
    let mut appears_twice = false;
    let mut groups: HashMap<String, usize> = HashMap::new();

    let chars: Vec<char> = input.chars().collect();

    for i in 0..chars.len() - 1 {
        // Create pair of two consecutive characters
        let group = chars[i].to_string() + &chars[i + 1].to_string();

        // Check for non-overlapping pairs
        if let Some(&prev_index) = groups.get(&group) {
            if i > prev_index + 1 {
                appears_twice = true;
            }
        }
        groups.insert(group, i);

        // Check for repeating characters with one character in between
        if i >= 2 && chars[i] == chars[i - 2] {
            repeated_between = true;
        }

        // Early exit if both conditions are satisfied
        if repeated_between && appears_twice {
            break;
        }
    }

    repeated_between && appears_twice
}

fn classify_strings(path: String) -> usize {
    let strings_list = extract_strings(path);
    strings_list
        .iter()
        .filter(|word| filter_string(word) && second_filtering(word))
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
