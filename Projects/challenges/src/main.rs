use std::collections::HashMap;
use std::time::Instant;

mod challenge_1;
mod challenge_2;
use crate::challenge_1::lettersum;
use crate::challenge_2::{
    assign_value, find_word_sum, find_word_sum_improved, find_word_sum_iterative, word_group_by_sum,
};

fn main() {
    // Create new HasMap
    let mut char_map: HashMap<char, i32> = HashMap::new();

    // fill HashMap with letter and its value
    for (i, letter) in ('a'..='z').enumerate() {
        char_map.insert(letter, i as i32 + 1);
    }

    // CHALLENGES RESOLUTION
    // Challenge 1
    println!(
        "Challenge 1 result: {}",
        lettersum("microspectrophotometries", &char_map)
    );

    // Challenge 2
    println!("Challenge 2 result: {:?}", find_word_sum(313, &char_map));

    // Challenge 2.1
    // Assign a value to each word in the word list before read the sum, this way you can delimit your search
    println!(
        "Challenge 2.1 result: {:?}",
        word_group_by_sum(5, &char_map)
    );

    // Challenge 3
    //println!("Challenge 3 result: {}", odd_words(&charmap));

    // TESTING
    // some testing for challenge 1
    {
        assert_eq!(lettersum("abcd", &char_map), 10);
        assert_eq!(lettersum("", &char_map), 0);
        assert_eq!(lettersum("a", &char_map), 1);
        assert_eq!(lettersum("z", &char_map), 26);
        assert_eq!(lettersum("cab", &char_map), 6);
        assert_eq!(lettersum("excellent", &char_map), 100);
        assert_eq!(lettersum("microspectrophotometries", &char_map), 317);

        // some testing for challenge 2
        dbg!("Original sum");
        let start = Instant::now();

        assert_eq!(find_word_sum(313, &char_map), ["polytetrafluoroethylene"]);
        assert_eq!(find_word_sum(1, &char_map), ["a"]);
        assert_eq!(find_word_sum(2, &char_map), ["aa", "b"]);

        let duration = start.elapsed();
        println!("Time elapsed: {:?}", duration);

        dbg!("Attempting to pre-cache every word in the dictionary");

        let cached_words = assign_value(&char_map);
        assert_eq!(cached_words.is_empty(), false);
        assert_eq!(
            cached_words
                .get(&313)
                .unwrap()
                .contains(&String::from("polytetrafluoroethylene")),
            true
        );

        dbg!("Calling 'improved' partial sum");

        let start = Instant::now();

        assert_eq!(
            find_word_sum_improved(313, &char_map),
            ["polytetrafluoroethylene"]
        );
        assert_eq!(find_word_sum_improved(1, &char_map), ["a"]);
        assert_eq!(find_word_sum_improved(2, &char_map), ["aa", "b"]);

        let duration = start.elapsed();
        println!("Time elapsed: {:?}", duration);

        dbg!("Calling iterative partial sum");

        let start = Instant::now();

        assert_eq!(
            find_word_sum_iterative(313, &char_map),
            ["polytetrafluoroethylene"]
        );
        assert_eq!(find_word_sum_iterative(1, &char_map), ["a"]);
        assert_eq!(find_word_sum_iterative(2, &char_map), ["aa", "b"]);

        let duration = start.elapsed();
        println!("Time elapsed: {:?}", duration);
        // some testing for challenge 3
    }
}

// Challenge 3: How many words have an odd letter sum?

//Function that return those words wich sum is an odd number

//fn odd_words(charmap: &HashMap<char, i32>) -> i32 {
//let words_list = lines_from_file("./words_alpha.txt");
//words_list.into_iter().filter(|odd_word| lettersum(odd_word, charmap) % 2 == 0).collect().
//}
