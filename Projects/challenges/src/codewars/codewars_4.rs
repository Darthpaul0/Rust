/**
 * The drawing shows 6 squares the sides of which have a length of
 * 1, 1, 2, 3, 5, 8. It's easy to see that the sum of the perimeters
 * of these squares is : 4 * (1 + 1 + 2 + 3 + 5 + 8) = 4 * 20 = 80
 * https://www.codewars.com/kata/559a28007caad2ac4e000083
 */

pub fn fib_sequence(n: usize) -> Vec<usize> {
    // here store the sequence
    let mut sequence: Vec<usize> = Vec::new();
    let mut index = 0;
    let mut sum;

    match n {
        0 => return vec![0],
        1 => return vec![1],
        n => {
            // insert first 2 numbers
            sequence.push(0);
            sequence.push(1);

            // start calculating the sequence
            while index < n {
                sum = sequence.get(sequence.len() - 1).unwrap()
                    + sequence.get(sequence.len() - 2).unwrap();

                // add sum to the sequence
                sequence.push(sum);
                index += 1;
            }
            sequence
        }
    }
}

pub fn perimeter(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        n => {
            let sequence: usize = fib_sequence(n).iter().sum();
            sequence * 4
        }
    }
}
