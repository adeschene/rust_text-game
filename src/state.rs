use std::fs;

/// Represents the gamestate as a collection
/// of information about the current game.
///
/// This will eventually include flags for
/// denoting which rooms are accessible,
/// which items have been picked up, etc.

pub struct State {
    pub curr_room: usize,
    pub in_final_room: bool,
}

impl State {
    pub fn new() -> State {
        State {
            curr_room: 0,
            in_final_room: false,
        }
    }
    /// Takes a path to the save file and returns the
    /// game state that can be built from the info in that file.
    pub fn deserialize(path: &str) -> State {
        let gamedata = fs::read_to_string(path).unwrap();
        let mut entries = gamedata.lines();
        State {
            curr_room: entries.next().unwrap().parse().unwrap(),
            in_final_room: entries.next().unwrap().parse().unwrap(),
        }
    }
    /// Converts itself into a String representation for saving.
    pub fn serialize(&self) -> String {
        let s = format!("{}\n{}\n"
                        , &self.curr_room.to_string()
                        , &self.in_final_room.to_string()
                       );
        s
    }
    /// Keeps track of various flags regarding game behavior,
    /// player decisions, actions, etc. Very limited currently.
    pub fn update_flags(self, final_room: usize) -> State {
        State {
            curr_room: self.curr_room,
            in_final_room: &self.curr_room == &final_room,
        }
    }
}















