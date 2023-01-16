/**
 * In this file we will create all functions related to the player insertion
 */

 use std::sync::atomic::{AtomicU32, Ordering};
// Struct to create a player
pub struct Player {
    id: u32,
    wins: i32,
    loss: i32,
    eliminated: bool
}


// implementation for players

// we are using this type because it is safe to use in multithread
static NEXT_ID: AtomicU32 = AtomicU32::new(1);
/* Create new player with default values*/
impl Player {
    fn new () -> Player {
        Player {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            wins: 0,
            loss: 0,
            eliminated: false
        }
    }
}

/** Method to create a determined number of players and
 *  store them in a Vector 
 */
 pub fn insert_players(num_players: i32)->Vec<Player>{
    let players: Vec<Player> = Vec::new();
    // 1. Read user input for number of players

    // 2. Check if it is a valid number
    // 3. Return a vector with the number of players.
        // Each player is has a number of wins, losses and eliminate status

    players
 }

 fn read_user_input() -> i32{
    5
 }