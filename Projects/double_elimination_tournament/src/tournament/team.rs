/* Functions related to team */

use super::player::Player;
use std::sync::atomic::{AtomicU32, Ordering};

/**
Basic structure for a team
- id: unique identification
- name: optional
- players: number of players in the team
    - we don't need to specify a field named eliminated or status,
    since we will remove the team once the number of players in it equals 0
*/
#[derive(Debug)]
// the lifetime must be specified since a team cannot outlive a player
pub struct Team<'a> {
    id: u32,
    name: Option<String>,
    players: Vec<&'a Player>,
}

// implementations for team
static NEXT_ID: AtomicU32 = AtomicU32::new(1);
impl<'a> Team<'a> {
    /** Create new team */
    pub fn new(name: &str, players: Vec<&'a Player>) -> Self {
        Team {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            name: Some(String::from(name)),
            players,
        }
    }
    /** Add a player to the team */
    pub fn add_player(&mut self, player: &'a Player) {
        self.players.push(player)
    }

    /** Remove player from the team */
    pub fn remove_player(&mut self, player: &Player) {
        // search and remove the player
        if let Some(posicion) = self.players.iter().position(|&p| p == player) {
            self.players.remove(posicion);
        }
    }

    /** This function is used to calculate the ELO of a team by using a geometric average.
    The idea is to compensate the team level when there are players in a team with
    a really different ELO (such as 2000 and 500)
    */
    fn avg_elo(self) -> f64 {
        // get all elos in the team
        let all_elos: Vec<_> = self.players.iter().map(|&x| x.elo()).collect();

        // get number of elos (which will be equal to the number of players)
        let n = all_elos.len() as f64;

        // process of compensation
        let n_roots_of_elo: Vec<_> = all_elos.iter().map(|x| (*x as f64).powf(1.0 / n)).collect();
        n_roots_of_elo.iter().product()
    }
}
// Some testing
#[cfg(test)]
mod test {
    use crate::tournament::player::{self, Player};

    use super::Team;

    #[test]
    fn test_team_elo() {
        // create test players
        let player_1 = Player::new("Paco", 200);
        let player_2 = Player::new("Paco", 200);
        let player_3 = Player::new("Paco", 200);

        // create test team
        let mut test_team = Team::new("ermejo", vec![]);

        test_team.add_player(&player_1);
        test_team.add_player(&player_2);
        test_team.add_player(&player_3);

        // allowed margin of error
        let epsilon = 0.001;

        assert!((test_team.avg_elo() - 200.0).abs() < epsilon);
    }

    #[test]
    fn test_team_elo_big_differences() {
        // create test players
        let player_1 = Player::new("Paco", 2000);
        let player_2 = Player::new("Paco", 9);
        let player_3 = Player::new("Paco", 1);

        // create test team
        let mut test_team = Team::new("ermejo", vec![]);

        test_team.add_player(&player_1);
        test_team.add_player(&player_2);
        test_team.add_player(&player_3);

        let epsilon = 0.001;

        assert!((test_team.avg_elo() - 26.2074).abs() < epsilon);
    }

    #[test]
    fn test_add_player() {
        // create test team
        let mut test_team = Team::new("ermejo", vec![]);

        // create test players
        let player_1 = Player::new("Paco", 200);

        // get number of players of the team
        let num_players = test_team.players.len();

        // add player
        test_team.add_player(&player_1);

        // test
        assert!(num_players < test_team.players.len())
    }

    #[test]
    fn test_remove_player() {
        // create test players
        let player_1 = Player::new("Paco", 2000);
        let player_2 = Player::new("Paco", 9);
        let player_3 = Player::new("Paco", 1);

        // create test team
        let mut test_team = Team::new("ermejo", vec![]);

        // add players to the team
        test_team.add_player(&player_1);
        test_team.add_player(&player_2);
        test_team.add_player(&player_3);

        // get actual number of players
        let num_players = test_team.players.len();

        // remove several players
        test_team.remove_player(&player_1);
        test_team.remove_player(&player_2);

        // test
        assert!(num_players > test_team.players.len())
    }
}
