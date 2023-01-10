use std::collections::HashMap;

/**
 * Given a list of integers and a single sum value,
 * return the first two values(parse from the left please)
 * in order of appearance that add up to form the sum.

 * If there are two or more pairs with the required sum,
 * the pair whose second element has the smallest index is the solution.

 * Negative numbers and duplicate numbers can and will appear!

 * Complete problem with example here --> https://www.codewars.com/kata/54d81488b981293527000c8f
*/

pub fn sum_pairs(numbers: Vec<i32>, sum: i32) -> Vec<i32> {
    let correct_pair: Vec<i32> = Vec::new();

    // aux
    let mut first_position;
    let mut second_position;
    // loop over the numbers vector
    for first in numbers.iter() {
        // compare it with next numbers
        // skipping first number to avoid repeating
        for second in numbers.iter() {
            // NOT WORKING
            // POSITION METHOD FAILS TO RETURN THE CORRECT INDEXES

            first_position = numbers.iter().position(|pos| pos == first).unwrap();
            second_position = numbers.iter().position(|pos| pos == second).unwrap();
            println!("[{},{}]", first_position, second_position);
        }
    }
    correct_pair
}
