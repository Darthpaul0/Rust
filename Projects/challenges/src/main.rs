// this way cargo will stop bothering us with warnings!
#![allow(dead_code, unused_imports)]

use std::collections::HashMap;
use std::time::Instant;

// imports
mod challenges;
mod codewars;
use codewars::{codewars_7::parentheses, codewars_8::alternative_add};

use crate::{
    challenges::{
        challenge_1::lettersum,
        challenge_2::*,
        challenge_3::odd_words,
        challenge_4::most_common_sum,
        challenge_5::*,
        challenge_6::{words_diff_letters, words_diff_letters_v2},
        challenge_7::longest_list,
        challenge_8::create_diamond,
    },
    codewars::{codewars_1::*, codewars_2::cmp_same, codewars_3::*, codewars_4::*, codewars_6::*},
};

fn main() {
    // Create new HasMap
    let mut char_map: HashMap<char, i32> = HashMap::new();

    // fill HashMap with letter and its value
    for (i, letter) in ('a'..='z').enumerate() {
        char_map.insert(letter, i as i32 + 1);
    }

    // CHALLENGES RESOLUTION
    {
        // Challenge 1
        // println!(
        //     "Challenge 1 result >>> {}",
        //     lettersum("microspectrophotometries", &char_map)
        // );

        // Challenge 2
        // println!("Challenge 2 result >>> {:?}", find_word_sum(313, &char_map));

        // Challenge 2.1
        // Assign a value to each word in the word list before read the sum, this way you can delimit your search
        // println!(
        //     "Challenge 2.1 result >>> {:?}",
        //     word_group_by_sum(313, &char_map)
        // );

        // Challenge 2.2
        // Read each character of the word and stops if its greater than the specified sum
        // println!(
        //     "Challenge 2.2 result >>> {:?}",
        //     find_word_sum_improved(313, &char_map)
        // );

        // Challenge 3
        // println!(
        //     "Challenge 3 result >>> There are {} odd words",
        //     odd_words(&char_map)
        // );

        // Challenge 4
        // println!(
        //     "Challenge 4 result >>> The most common letter sum is: {:?}, there are {:?} words",
        //     lettersum(most_common_sum(&char_map).get(0).unwrap(), &char_map),
        //     most_common_sum(&char_map).len()
        // );

        // Challenge 5
        // println!(
        // "Challenge 5 result >>> {:?}",
        // words_equals_separated(132, 5, &char_map)
        // );
        // println!(
        // "Challenge 5 result (v2) >>> {:?}",
        // words_equals_separated_v2(11, &char_map)
        // );

        // Challenge 6
        // println!("Challenge 6 result >>> {:?}", words_diff_letters(&char_map));
        // println!(
        //     "Challenge 6 result >>> {:?}",
        //     words_diff_letters_v2(&char_map)
        // );

        // Challenge 7
        // println!("{:?}", longest_list());

        // Challenge 8
        // println!("{}", create_diamond(7));
    }

    // CODEWARS CHALLENGES RESOLUTION
    {
        // Codewars 1
        // println!(
        // "Codewars 1 result >>> {:?}",
        // sum_pairs(vec![10, 5, 2, 3, 7, 5], 10)
        // );

        // Codewars 2
        // println!(
        // "Codewars 2 result >>> {:?}",
        // cmp_same(
        // vec![121, 144, 19, 161, 19, 144, 19, 11],
        // vec![121, 14641, 20736, 361, 25921, 361, 20736, 361]
        // )
        // )

        // Codewars 3
        // println!("Last survivor is: {}", josephus_survivor(7, 2))

        // Codewars 4
        // println!("Codewars 4 result >>> {:?}", perimeter(7))

        // Codewars 5 is actually challenge 8!

        // Codewars 6
        // println!("Codewars 6 >>> {:?}", product_fib(714));
        // println!("Codewars 6 >>> {:?}", product_fib(800))

        // Codewars 7
        // println!(
        // "Codewars 7 >>> {}",
        // parentheses("(()())()()()((()))))".to_string())
        // )

        // Codewars 8
        println!("Codewars 8 >>> {}", alternative_add(5, 19));
        println!("Codewars 8 >>> {}", alternative_add(-27, 18));
        println!("Codewars 8 >>> {}", alternative_add(-14, -16));
    }

    // TESTING
    {
        // some testing for challenge 1
        //     assert_eq!(lettersum("abcd", &char_map), 10);
        //     assert_eq!(lettersum("", &char_map), 0);
        //     assert_eq!(lettersum("a", &char_map), 1);
        //     assert_eq!(lettersum("z", &char_map), 26);
        //     assert_eq!(lettersum("cab", &char_map), 6);
        //     assert_eq!(lettersum("excellent", &char_map), 100);
        //     assert_eq!(lettersum("microspectrophotometries", &char_map), 317);

        //     // some testing for challenge 2
        //     dbg!("Original sum");
        //     let start = Instant::now();

        //     assert_eq!(find_word_sum(313, &char_map), ["polytetrafluoroethylene"]);
        //     assert_eq!(find_word_sum(1, &char_map), ["a"]);
        //     assert_eq!(find_word_sum(2, &char_map), ["aa", "b"]);

        //     let duration = start.elapsed();
        //     println!("Time elapsed: {:?}", duration);

        //     dbg!("Attempting to pre-cache every word in the dictionary");

        //     let cached_words = assign_value(&char_map);
        //     assert_eq!(cached_words.is_empty(), false);
        //     assert_eq!(
        //         cached_words
        //             .get(&313)
        //             .unwrap()
        //             .contains(&String::from("polytetrafluoroethylene")),
        //         true
        //     );

        //     dbg!("Calling 'improved' partial sum");

        //     let start = Instant::now();

        //     assert_eq!(
        //         find_word_sum_improved(313, &char_map),
        //         ["polytetrafluoroethylene"]
        //     );
        //     assert_eq!(find_word_sum_improved(1, &char_map), ["a"]);
        //     assert_eq!(find_word_sum_improved(2, &char_map), ["aa", "b"]);

        //     let duration = start.elapsed();
        //     println!("Time elapsed: {:?}", duration);

        //     dbg!("Calling iterative partial sum");

        //     let start = Instant::now();

        //     assert_eq!(
        //         find_word_sum_iterative(313, &char_map),
        //         ["polytetrafluoroethylene"]
        //     );
        //     assert_eq!(find_word_sum_iterative(1, &char_map), ["a"]);
        //     assert_eq!(find_word_sum_iterative(2, &char_map), ["aa", "b"]);

        //     let duration = start.elapsed();
        //     println!("Time elapsed: {:?}", duration);
        //     // some testing for challenge 3
    }
}
