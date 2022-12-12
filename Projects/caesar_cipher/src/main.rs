use cipher_crypt::Caesar;
use cipher_crypt::Cipher;
use scanrs::scann;
use std::io::stdin;
fn main() {
    println!("Ave, Rustacean!");
    println!("*****************");
    loop {
        println!("Select an option:");
        println!("1. Encrypt message");
        println!("2. Decipher message");
        println!("3. Exit programme");

        let option = scann();

        match option {
            1 => {
                println!("You selected option {}", option);
                println!("Insert the message to encrypt");
                let msg: String = scann();
                println!("Insert cipher key");

                let key = loop {
                    let mut input = String::new();
                    stdin()
                        .read_line(&mut input)
                        .expect("Unable to read from stdin");
                    // parse user input
                    match input.trim().parse::<usize>() {
                        // check if the input is correct and inside boundaries
                        Ok(key) if (1..=26).contains(&key) => break key,
                        Ok(_) => println!("Please, insert a number between 1 and 26"),
                        Err(e) => println!("{e}"),
                    };
                };

                let c = Caesar::new(key);
                let encrypted = c.encrypt(&msg);

                println!("Encrypted message: {}.", encrypted.unwrap());
                break;
            }
            2 => {
                println!("You selected option {}.", option);
                println!("Insert the message to decipher");
                let msg: String = scann();
                println!("Insert cipher key");
                let key = scann();
                let c = Caesar::new(key);
                let decrypted = c.decrypt(&msg);

                println!("Encrypted message: {}.", decrypted.unwrap());
                break;
            }
            3 => {
                println!("You selected option {}. Goodbye!", option);
                break;
            }
            _ => println!("No option selected. Please insert a valid option"),
        }
    }
}
