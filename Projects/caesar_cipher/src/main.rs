use text_io::read;

/**
 * ask user to:
 * 1. cipher a phrase
 *  - insert phrase and cipher
 * 2. decipher a phrase
 *  - insert phrase and cipher
 */

fn main() {
    println!("Ave, Rustacean!");
    println!("What do you want to do? Please select an option. \n 1. Cipher a phrase \n 2. Decipher a phrase");
    let option: i32 = read!();

    match option {
        1 => {
            println!("You selected option {}: Cipher a phrase", option);
            println!("{}",cipher_phrase());
        }
        2 => {
            println!("You selected option {}: Decipher a phrase", option)
        }
        _ => println!("No option selected"),
    };
}

fn cipher_phrase() -> String {
    let alphabet: String = String::from("abcdefghijklmnopeqrstuvwxyz");
    println!("Insert the cipher type");
    let cipher_tipe: i32 = read!();
    println!("Insert the phrase to cipher");
    let phrase_to_cipher: String = read!("{}\n");
    // we have to slice as many letters as the cipher_type
    phrase_to_cipher
}
