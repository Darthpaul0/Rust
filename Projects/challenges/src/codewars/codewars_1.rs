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

pub fn sum_pairs(numbers: &[i8], sum: i8) -> Option<(i8, i8)> {
    let mut correct_pair = None;

    // aux
    let mut index = 0;
    // loop over the numbers vector
    for first in numbers.iter() {
        // compare it with next numbers
        // skipping first number to avoid repeating
        for (pos, second) in numbers.iter().enumerate() {
            // print!(" {pos}");
            // print!(" {index}");
            if first + second == sum && pos < index {
                index = pos;
                correct_pair = Some((*first, *second));
                break;
            }
            index += 1;
        }
    }
    correct_pair
}
