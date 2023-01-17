mod features;

use crate::features::players::*;
fn main() {
    println!("Welcome to the programme!");

    // Request number os players
    let players = insert_players();

    // check number of players
    match players.len() {
        0 => println!("No players added. The tournament won't take place :("),
        _ => println!(
            "{:?} players will participate in the tournament. Good luck!",
            players.len()
        ),
    }

    
}
