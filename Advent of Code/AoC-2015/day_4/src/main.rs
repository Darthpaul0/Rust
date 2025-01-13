use std::time::Instant;

fn main() {
    println!("{:?}", find_number("iwrupvqb".to_string()));
}

// How can I decypher MD5
// So, create a digest, and find out which is the lower number that combined with
// the input returns a hash that starts with 5 zeroes (like 000006136ef2ff3b291c85725f17325c)

fn find_number(input: String) -> i32 {
    let mut counter = 0;
    let time = Instant::now();
    println!("Searching for the number...");
    loop {
        let resulting_hash = format!(
            "{:x}",
            md5::compute(input.clone() + &counter.to_string())
        );
        if resulting_hash.starts_with("000000") {
            break;
        } else {
            if counter % 500000 == 0 {
                println!("None of the {} numbers tried, still searching...", counter);
            }
            counter += 1;
        }
    }
    println!("Number found! It was {}", counter);
    println!("Time elapsed: {:?}", time.elapsed());
    counter
}
