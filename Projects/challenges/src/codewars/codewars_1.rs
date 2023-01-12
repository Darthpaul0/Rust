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

pub fn sum_pairs_v2(values: Vec<i32>, sum: i32) {
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
