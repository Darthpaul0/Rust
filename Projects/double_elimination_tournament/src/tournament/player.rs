use std::fmt::Display;
/**
 * In this file we will create all functions related to the player
 */
use std::sync::atomic::{AtomicU32, Ordering};
// Struct to create a player
#[derive(Debug)]
/**
 Each player will have the following fields:
 - id: unique identification
 - name: optional
 - wins: to determine the ownership to which bracket
 - loss: to filter players
 - elo: to classify the players and the teams
 */
pub struct Player {
    id: u32,
    name: Option<String>,
    wins: i32,
    loss: i32,
    elo: usize,
}

// implementation for players

// we are using this type because it is safe to use in multithread
static NEXT_ID: AtomicU32 = AtomicU32::new(1);
impl Player {
    /** Create new player*/
    pub fn new(name: &str, elo: usize) -> Self {
        Player {
            name: Some(String::from(name)),
            elo,
            ..Default::default()
        }
    }
}

// Trait to create default players
impl Default for Player {
    /** Create players with default values */
    fn default() -> Self {
        // Each player has a number of wins, losses and eliminate status
        Player {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            name: None,
            wins: 0,
            loss: 0,
            elo: 0,
        }
    }
}

// Trait to display the name of the players
impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(name) => write!(f, "{}", name),
            None => write!(f, "id:{}", self.id)
        }
    }
}