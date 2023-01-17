use super::codewars_4::fib_sequence;

/**
* Product of consecutive Fib numbers

* https://www.codewars.com/kata/5541f58a944b85ce6d00006a
*/

pub fn product_fib(num: usize) -> Vec<(usize, usize, bool)> {
    let mut result: Vec<(usize, usize, bool)> = Vec::new();
    let sequence = fib_sequence(80);
    let mut index = 0;
    // loop over the sequence multiplying numbers until it's equal or greater to the specified number
    for number in sequence.iter() {
        let next = sequence.get(index + 1).unwrap();
        if number * next == num {
            result.push((*number, *next, true));
            break;
        }
        if number * next > num {
            result.push((*number, *next, false));
            break;
        }
        index += 1;
    }

    result
}
