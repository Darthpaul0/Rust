/* Create all functions related to game */
#[derive(Debug)]
/** Possible states of a game.*/
#[derive(PartialEq)]
pub enum GameState {
    Planned,
    Ongoing,
    Cancelled,
    Finished,
}
/** Struct to handle games.*/
pub struct Game {
    state: GameState,
}
impl Game {
    pub fn new(state: GameState) -> Self {
        Game { state }
    }
    pub fn set_state(&mut self, state: GameState) -> &mut Game {
        self.state = state;
        self
    }
    pub fn get_state(self) -> GameState {
        self.state
    }
}

#[cfg(test)]
mod test {
    use super::{Game, GameState};

    fn test_set_result() {
        // create a game
        let game = Game::new(super::GameState::Planned);

        // change game result
        // game.set_state(GameState::Ongoing);

        // assert_eq!(&game.get_state(), &GameState::Ongoing)
    }
}
