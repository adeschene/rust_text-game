use std::fs;

/// Represents the gamestate as a collection
/// of information about the current game.
///
/// This will eventually include flags for
/// denoting which rooms are accessible,
/// which items have been picked up, etc.

pub struct State {
    pub curr_room: usize,
}

impl State {
    pub fn new() -> State {
        State {
            curr_room: 0,
        }
    }
    /// Takes a path to the save file and returns the
    /// game state that can be built from the info in that file.
    pub fn deserialize(path: &str) -> State {
        let gamedata = fs::read_to_string(path).unwrap();
        let mut entries = gamedata.lines();
        State {
            curr_room: entries.next().unwrap().parse().unwrap(),
        }
    }
    /// Converts itself into a String representation for saving.
    pub fn serialize(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.curr_room.to_string());
        s
    }
}
