use std::result;

/**
 * Given two arrays a and b write a function comp(a, b) (orcompSame(a, b))
 * that checks whether the two arrays have the "same" elements,
 * with the same multiplicities
 * (the multiplicity of a member is the number of times it appears).
 * "Same" means, here, that the elements in b are the elements in a squared,
 * regardless of the order.
 *
 * Complete problem with examples --> https://www.codewars.com/kata/550498447451fbbd7600041c
 */

pub fn cmp_same(first: Vec<i32>, second: Vec<i32>) -> bool {
    // check length of both list
    if first.len() != second.len() {
        return false;
    }
    // order and square first list
    let mut squared: Vec<i32> = first.iter().map(|x| x * x).collect();
    squared.sort();

    // order second list
    let mut ordered = second.clone();
    ordered.sort();
    // check if both vectors are equals
    if squared == ordered {
        return true;
    } else {
        return false;
    }
}
