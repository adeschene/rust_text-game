
/// Represents the gamestate as a collection
/// of information about the current game.
///
/// This will eventually include flags for
/// denoting which rooms are accessible,
/// which items have been picked up, etc.

pub struct State {
    pub curr_room: usize,
    pub examined_wall: bool,
    pub took_key: bool,
    pub took_broom: bool,
    pub took_nail: bool,
    pub met_blimpo: bool,
    pub final_room_unlocked: bool,
}

/// The implementation of the State struct.

impl State {

    /// Create a default, empty State.

    pub fn empty() -> State {
        State {
            curr_room: 0,
            examined_wall: false,
            took_key: false,
            took_broom: false,
            took_nail: false,
            met_blimpo: false,
            final_room_unlocked: false,
        }
    }

    /// Create a new State based on the passed in arguments.

    pub fn new(curr_room: usize, examined_wall: bool, took_key: bool,
               took_broom: bool, took_nail: bool, met_blimpo: bool,
               final_room_unlocked: bool) -> State {
        State {
            curr_room,
            examined_wall,
            took_key,
            took_broom,
            took_nail,
            met_blimpo,
            final_room_unlocked,
        }
    }

    /// Takes a path to the save file and returns the
    /// game state that can be built from the info in that file.

    pub fn deserialize(save_data: &str) -> State {
        let mut entries = save_data.lines();
        State {
            curr_room: entries.next().unwrap().parse().unwrap(),
            examined_wall: entries.next().unwrap().parse().unwrap(),
            took_key: entries.next().unwrap().parse().unwrap(),
            took_broom: entries.next().unwrap().parse().unwrap(),
            took_nail: entries.next().unwrap().parse().unwrap(),
            met_blimpo: entries.next().unwrap().parse().unwrap(),
            final_room_unlocked: entries.next().unwrap().parse().unwrap(),
        }
    }

    /// Converts a State object into a String representation for saving.

    pub fn serialize(&self) -> String {
        let s = format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n~"
                        , &self.curr_room.to_string()
                        , &self.examined_wall.to_string()
                        , &self.took_key.to_string()
                        , &self.took_broom.to_string()
                        , &self.took_nail.to_string()
                        , &self.met_blimpo.to_string()
                        , &self.final_room_unlocked.to_string()
                       );
        s
    }

    /// Allows the game state to be updated more easily.
    /// Affects game behavior, player decisions, actions, etc.
    /// Cannot set the current room with this function.

    pub fn update(&self, change: (u8, bool)) -> State {
        match change.0 {
            1     =>
                State {
                    curr_room: self.curr_room,
                    examined_wall: change.1,
                    took_key: self.took_key,
                    took_broom: self.took_broom,
                    took_nail: self.took_nail,
                    met_blimpo: self.met_blimpo,
                    final_room_unlocked: self.final_room_unlocked,
                },
            2     =>
                State {
                    curr_room: self.curr_room,
                    examined_wall: self.examined_wall,
                    took_key: change.1,
                    took_broom: self.took_broom,
                    took_nail: self.took_nail,
                    met_blimpo: self.met_blimpo,
                    final_room_unlocked: self.final_room_unlocked,
                },
            3     =>
                State {
                    curr_room: self.curr_room,
                    examined_wall: self.examined_wall,
                    took_key: self.took_key,
                    took_broom: change.1,
                    took_nail: self.took_nail,
                    met_blimpo: self.met_blimpo,
                    final_room_unlocked: self.final_room_unlocked,
                },
            4     =>
                State {
                    curr_room: self.curr_room,
                    examined_wall: self.examined_wall,
                    took_key: self.took_key,
                    took_broom: self.took_broom,
                    took_nail: change.1,
                    met_blimpo: self.met_blimpo,
                    final_room_unlocked: self.final_room_unlocked,
                },
            5     =>
                State {
                    curr_room: self.curr_room,
                    examined_wall: self.examined_wall,
                    took_key: self.took_key,
                    took_broom: self.took_broom,
                    took_nail: self.took_nail,
                    met_blimpo: change.1,
                    final_room_unlocked: self.final_room_unlocked,
                },
            6     =>
                State {
                    curr_room: self.curr_room,
                    examined_wall: self.examined_wall,
                    took_key: self.took_key,
                    took_broom: self.took_broom,
                    took_nail: self.took_nail,
                    met_blimpo: self.met_blimpo,
                    final_room_unlocked: change.1,
                },
            _     =>
                State {
                    curr_room: self.curr_room,
                    examined_wall: self.examined_wall,
                    took_key: self.took_key,
                    took_broom: self.took_broom,
                    took_nail: self.took_nail,
                    met_blimpo: self.met_blimpo,
                    final_room_unlocked: self.final_room_unlocked,
                },
        }
    }
}















