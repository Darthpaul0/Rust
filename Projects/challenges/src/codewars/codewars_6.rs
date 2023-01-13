use super::codewars_4::fib_sequence;

/**
* Product of consecutive Fib numbers

* https://www.codewars.com/kata/5541f58a944b85ce6d00006a
*/

pub fn product_fib(num: i32) -> Vec<(i32, i32, bool)> {
    let mut result: Vec<(i32, i32, bool)> = Vec::new();
    let sequence = fib_sequence(num);

    // loop over the sequence
    // when we found a couple of consecutive numbers that
    // are correct, break and return

    sequence
        .iter()
        .map(|x| sequence.iter().map(|y| x.clone() + y == num));
    // let mut index = 1;
    /*
    for n in sequence.iter() {
        println!("{}", n);
        let first = sequence.get(index).unwrap();
        let second = sequence.get((index - 1) as usize).unwrap();
        if first * second == num {
            result = vec![(*first, *second, true)];
            break;
        }
        index += 1;
    }

    result */
    vec![]
}
