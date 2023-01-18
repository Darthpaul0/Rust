/**
Given Two integers a , b , find The sum of them , BUT You are not allowed to use the operators + and -
*/

pub fn alternative_add(a: i32, b: i32) -> i32 {
    // declare two vecs
    let a: Vec<i32> = vec![a];
    let b: Vec<i32> = vec![b];

    // apply wrapping_add instead of addition
    a[0].wrapping_add(b[0])
}
