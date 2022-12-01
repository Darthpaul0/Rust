use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first_number: String = args.nth(1).unwrap();
    let operator: char = args.nth(0).unwrap().chars().next().unwrap();
    let second_number: String = args.nth(0).unwrap();

    let first = first_number.parse::<f32>().unwrap();
    let second = second_number.parse::<f32>().unwrap();
    let result = operate(operator, first, second);

    println!("{}", output(first, operator, second, result));
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    format!(
        "{} {} {} = {}",
        first_number, operator, second_number, result
    )
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' | 'X' | 'x' => first_number * second_number,
        _ => panic!("Invalid operator used."),
    }
}

#[cfg(test)]
mod tests {
    use crate::operate;
    use crate::output;

    #[test]
    fn output_accepts_floating_point_numbers() {
        let out = output(-10.0, '*', 10.0, 0.0);
        assert_eq!(out, String::from("-10 * 10 = 0"));
    }
    #[test]
    fn operate_accepts_floating_point_numbers() {
        let op = operate('+', -10.5, 10.0);
        assert_eq!(op, -0.5);
    }

    #[test]
    fn output_expects_four_args() {
        let out = output(-10.0, '+', 10.0, 0.0);
        assert_eq!(out, String::from("-10 + 10 = 0"));
    }

    #[test]
    fn operate_handles_addition() {
        let op = operate('+', -5.0, 200.0);
        assert_eq!(op, 195.0);
    }
    #[test]
    fn operate_handles_subtraction() {
        let op = operate('-', -12.0, -12.0);
        assert_eq!(op, 0.0);
    }
    #[test]
    fn operate_handles_division() {
        let op = operate('/', -12.0, -1.0);
        assert_eq!(op, 12.0);
    }
    #[test]
    fn operate_handles_multiplication() {
        let op = operate('*', -12.0, -2.0);
        assert_eq!(op, 24.0);
    }
    #[test]
    fn operate_handles_multiplication_x() {
        let op = operate('x', -12.0, 2.0);
        assert_eq!(op, -24.0);
    }
    #[test]
    fn operate_handles_multiplication_cap_x() {
        let op = operate('X', -12.0, 2.0);
        assert_eq!(op, -24.0);
    }
    #[test]
    #[should_panic]
    fn operate_panics_on_invalid_op() {
        operate('a', 1.0, 1.0);
    }
}
