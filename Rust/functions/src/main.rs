// https://doc.rust-lang.org/book/ch03-03-how-functions-work.html

fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}