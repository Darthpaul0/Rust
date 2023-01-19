/* Create all functions related to game */
#[derive(Debug)]
/** Possible results of a game.*/
pub enum Result {
    Won,
    Lose,
    Draw,
}
/** Struct to handle games.*/
pub struct Game {
    result: Result,
}
impl Game {
    pub fn new(result: Result) -> Self {
        Game { result }
    }
    pub fn set_result(result: Result) -> Self {
        Game { result }
    }
    pub fn get_result(self) -> Result {
        self.result
    }
}

#[cfg(test)]
mod test {
    use super::Game;

    fn test_set_result() {
        // create a game
        let game = Game::new(super::Result::Lose);

        // get actual game result
        let result = game.get_result();

        // change game result
    }
}
