// Find the correct floor problem
// Open parenthesis -(- make Santa go up one floor
// Close parenthesis -)- make Santa go down one floor
// We have to find the correct floor for him

pub fn find_correct_floor(instructions: String) -> i32 {
    let mut counter = 0;
    for letter in instructions.chars() {
        if letter == '(' {
            counter += 1;
        } else if letter == ')' {
            counter -= 1;
        }
    }
    counter
}

pub fn enters_basement(instructions: String) -> i32 {
    let mut counter = 0;
    let mut position = 0;
    for (index, letter) in instructions.chars().enumerate() {
        if letter == '(' {
            counter += 1;
        } else if letter == ')' {
            counter -= 1;
        }
        if counter == -1 {
            position = index;
            break;
        }
    }
    position as i32 + 1
}
