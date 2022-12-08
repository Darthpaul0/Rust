extern crate rand;
use rand::Rng;

use std::fs::File;
use std::io::prelude::*;

use std::io;

const ALLOWED_ATTEMPS: u8 = 7;
struct Letter {
    character: char,
    revealed: bool,
}

enum GameProgress {
    InProgress,
    Won,
    Lost,
}

fn main() {
    let mut turns_left = ALLOWED_ATTEMPS;
    // get a word from words.txt
    let selected_word = selected_word();

    // ger letters from the word
    let mut letters = create_letters(&selected_word);

    println!("Welcome to Hangman Game");

    loop {
        println!("You have {} turns left", turns_left);
        display_progress(&letters);

        println!("Please enter a letter.");
        let user_char = read_user_input_character();

        // exit if user enters an asterisk
        if user_char == '*' {
            break;
        }

        // check if user's guess was correct
        let mut at_least_one_revealed = false;

        // updates the revealed state of each letter
        for letter in letters.iter_mut() {
            if letter.character == user_char {
                letter.revealed = true;
                at_least_one_revealed = true;
            }
        }

        // if user guessed wrong, lose a turn
        if !at_least_one_revealed {
            turns_left -= 1;
        }

        // check game progression
        match check_progress(turns_left, &letters) {
            GameProgress::InProgress => continue,
            GameProgress::Won => {
                println!("\nYou won! The word was: {}", selected_word);
                break;
            }
            GameProgress::Lost => {
                println!("\nYou lose! The word was: {}", selected_word);
                break;
            }
        }
    }

    println!("\nThanks for playing!");
}

fn selected_word() -> String {
    // open file
    let mut file = File::open("words.txt").expect("Cannot open file");

    // load file contents
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("Cannot read file");

    // get individual words
    let available_words: Vec<&str> = file_content.trim().split(',').collect();

    // generate random index

    let random_index = rand::thread_rng().gen_range(0, available_words.len());

    return String::from(available_words[random_index]);
}

fn create_letters(word: &String) -> Vec<Letter> {
    // create empty vector
    let mut letters: Vec<Letter> = Vec::new();

    // wrap each character in a letter struct
    for letter in word.chars() {
        letters.push(Letter {
            character: letter,
            revealed: false,
        });
    }

    return letters;
}

fn display_progress(letters: &Vec<Letter>) {
    let mut display_string = String::from("Progress:");

    // display the appropriate character for each letter (letter or _)
    for letter in letters {
        display_string.push(' ');

        if letter.revealed {
            display_string.push(letter.character);
        } else {
            display_string.push('_');
        }

        display_string.push(' ');
    }

    println!("{}", display_string);
}

fn read_user_input_character() -> char {
    let mut user_input = String::new();

    // read user input
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => match user_input.chars().next() {
            Some(c) => {
                return c;
            }
            None => {
                return '*';
            }
        },
        Err(_) => {
            return '*';
        }
    }
}

fn check_progress(turns_left: u8, letters: &Vec<Letter>) -> GameProgress {
    // determine if all letters have been revealed
    let mut all_revealed = true;
    for letter in letters {
        if !letter.revealed {
            all_revealed = false;
        }
    }

    if all_revealed {
        return GameProgress::Won;
    }

    if turns_left > 0 {
        return GameProgress::InProgress;
    }

    return GameProgress::Lost;
}
