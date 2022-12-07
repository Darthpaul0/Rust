#[derive(Debug)]
/**
 * Define a public struct named Args which consists of three public fields
 * of type String: image_1, image_2, and output
 */
pub struct Args {
    pub image_1: String,
    pub image_2: String,
    pub output: String,
}

impl Args {
    pub fn new() -> Self {
        Args {
            image_1: get_nth_arg(1),
            image_2: get_nth_arg(2),
            output: get_nth_arg(3),
        }
    }
}

/**
 * takes a usize, n, and returns a String
 * call the args function, and chain the nth method to get
 * the nth argument, unwrapping the value
*/
fn get_nth_arg(n: usize) -> String {
    std::env::args().nth(n).unwrap()
}
