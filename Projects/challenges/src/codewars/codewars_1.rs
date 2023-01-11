use std::{collections::HashMap, ops::Index, slice::SliceIndex};

/**
 * Given a list of integers and a single sum value,
 * return the first two values(parse from the left please)
 * in order of appearance that add up to form the sum.

 * If there are two or more pairs with the required sum,
 * the pair whose second element has the smallest index is the solution.

 * Negative numbers and duplicate numbers can and will appear!

 * Complete problem with example here --> https://www.codewars.com/kata/54d81488b981293527000c8f
*/

pub fn sum_pairs(ints: &[i8], s: i8) -> Option<Vec<(i8, i8, usize)>> {
    let mut pairs = None;
    let mut result: Vec<(i8, i8, usize)> = Vec::new();
    let mut index = 0;
    for i in ints {
        for j in ints[index + 1..].iter() {
            if i + j == s {
                println!("{}", index);
                result.push((*i, *j, index));
                pairs = Some(result.clone());
                break;
            }
        }
        index += 1;
    }
    pairs
}
