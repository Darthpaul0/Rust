/**
Write a function that takes a string of parentheses,
and determines if the order of the parentheses is valid.
The function should return true if the string is valid,
and false if it's invalid.
* Examples
"()"              =>  true
")(()))"          =>  false
"("               =>  false
"(())((()())())"  =>  true
* Constraints
0 <= input.length <= 100
*/

pub fn parentheses(input: String) -> bool {
    let mut result = false;

    // check length: odd numbers are instantly incorrect
    // check constraints
    if input.len() % 2 != 0 || input.len() > 100 || input.len() == 0 {
        return result;
    } else {
        let mut counter_a = 0;
        let mut counter_b = 0;

        // assign a type to each parenthesis and compare the number of each one
        for char in input.chars() {
            match char {
                ')' => counter_a += 1,
                '(' => counter_b += 1,
                _ => {}
            }
        }

        if counter_a == counter_b {
            result = true;
        }
    }
    result
}
