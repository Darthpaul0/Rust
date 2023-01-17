use std::io;
/**
 * In this file we will create all functions related to the player insertion
 */
use std::sync::atomic::{AtomicU32, Ordering};
// Struct to create a player
#[derive(Debug)]
pub struct Player {
    id: u32,
    wins: i32,
    loss: i32,
    eliminated: bool,
}

// implementation for players

// we are using this type because it is safe to use in multithread
static NEXT_ID: AtomicU32 = AtomicU32::new(1);
/* Create new player with default values*/
impl Player {
    fn new() -> Player {
        // Each player has a number of wins, losses and eliminate status
        Player {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            wins: 0,
            loss: 0,
            eliminated: false,
        }
    }
}

/** Method to create a determined number of players and
 *  store them in a Vector
 */
pub fn insert_players() -> Vec<Player> {
    let mut players: Vec<Player> = Vec::new();
    // 1. Read user input for number of players
    println!("Please insert the number of players that will participate in the tournament");
    let total_players: i32 = get_input();

    // 3. Return a vector with the number of players.
    let mut index = 0;
    while index < total_players {
        // create the new player
        let new_player = Player::new();

        // store player in the Vec
        players.push(new_player);

        index += 1;
    }
    players
}

/** Generic function to get the user input, no matter what input is inserted */
pub fn get_input<U: std::str::FromStr>() -> U {
    loop {
        let mut input = String::new();

        // Reads the input from STDIN and places it in the String named input.
        println!("Enter a value:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        // Convert to another type.
        // If successful, bind to a new variable named input.
        // If failed, restart the loop.
        let input = match input.trim().parse::<U>() {
            Ok(parsed_input) => parsed_input,
            Err(_) => {
                println!("Invalid input!");
                continue;
            }
        };
        return input;
    }
}