use std::{
    collections::{HashMap, HashSet},
    ops::Index,
    slice::SliceIndex,
};

/**
 * Given a list of integers and a single sum value,
 * return the first two values(parse from the left please)
 * in order of appearance that add up to form the sum.

 * If there are two or more pairs with the required sum,
 * the pair whose second element has the smallest index is the solution.

 * Negative numbers and duplicate numbers can and will appear!

 * Complete problem with example here --> https://www.codewars.com/kata/54d81488b981293527000c8f
*/

pub fn sum_pairs(values: Vec<i32>, sum: i32) {
    let mut pair_index: HashSet<((i32, i32), (usize, usize))> = HashSet::new();

    for (i, &x) in values.iter().enumerate() {
        for (j, &y) in values.iter().enumerate() {
            if x + y == sum && i < j {
                pair_index.insert(((x, y), (i, j)));
            }
        }
    }

    let mut result: (i32, i32) = (0, 0);
    let smallest_index = values.len();

    for (pair, indexes) in pair_index.into_iter() {
        if indexes.1 < smallest_index {
            result.0 = pair.0;
            result.1 = pair.1;
        }
    }

    dbg!(&result);
}
