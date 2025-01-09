/* Branch division between winners and losers */

use super::{game::Game, team::Team};

/** A bracket will have its own games, which will be played by teams in different rounds.*/
pub struct Bracket<'a> {
    games: Vec<Game>,
    teams: Vec<Team<'a>>,
    rounds: i32,
}

impl<'a> Bracket<'a> {
    // falta una clase Round?
    fn new() {}
}
