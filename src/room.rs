
// Denotes the end-game room.
pub const FINAL_ROOM: usize = 15;

/// Based on usize room index, returns the name
/// and description of the room.
///
/// # Panics
///
/// Will panic if given a room that can't be matched,
/// as all rooms must be accounted for.

pub fn get_info(room_index: usize) -> (String, String) {
    match room_index {
        0  => ("Cell".to_string(),
               "An empty cell with a single door.".to_string()),
        1  => ("Room1".to_string(),
               "Can go in any direction.".to_string()),
        2  => ("Room2".to_string(),
               "Another short hallway.".to_string()),
        3  => ("Room3".to_string(),
               "Can go east or west.".to_string()),
        4  => ("Room4".to_string(),
               "Can go east or west.".to_string()),
        5  => ("Room5".to_string(),
               "Can go east or west.".to_string()),
        6  => ("Room6".to_string(),
               "Can go east, north, or south.".to_string()),
        7  => ("Room7".to_string(),
               "Can go north or south.".to_string()),
        8  => ("Room8".to_string(),
               "Can go east or north.".to_string()),
        9  => ("Room9".to_string(),
               "Can go east or west.".to_string()),
        10 => ("Room10".to_string(),
               "Can go east or west.".to_string()),
        11 => ("Room11".to_string(),
               "Can go west.".to_string()),
        12 => ("Room12".to_string(),
               "Can go east or west.".to_string()),
        13 => ("Room13".to_string(),
               "Can go north or west.".to_string()),
        14 => ("Room14".to_string(),
               "Can go south.".to_string()),
        15 => ("Room15".to_string(),
               "Final room.".to_string()),
        _ => panic!() // We should be guaranteed an existing room.
    }
}

/// Move player to room immediately North of the
/// current room, or not, if there is no such room.

pub fn go_north(room_index: usize) -> usize {
    match room_index {
        0     => 1,
        1     => 2,
        2     => 3,
        6     => 15,
        7     => 6,
        8     => 7,
        13    => 14,
        other => other
    }
}

/// Move player to room immediately West of the
/// current room, or not, if there is no such room.

pub fn go_west(room_index: usize) -> usize {
    match room_index {
        1     => 10,
        3     => 4,
        4     => 5,
        5     => 6,
        9     => 8,
        10    => 9,
        11    => 1,
        12    => 3,
        13    => 12,
        other => other
    }
}

/// Move player to room immediately South of the
/// current room, or not, if there is no such room.

pub fn go_south(room_index: usize) -> usize {
    match room_index {
        1     => 0,
        2     => 1,
        3     => 2,
        6     => 7,
        7     => 8,
        14    => 13,
        15    => 6,
        other => other
    }
}

/// Move player to room immediately East of the
/// current room, or not, if there is no such room.

pub fn go_east(room_index: usize) -> usize {
    match room_index {
        1     => 11,
        3     => 12,
        4     => 3,
        5     => 4,
        6     => 5,
        8     => 9,
        9     => 10,
        10    => 1,
        12    => 13,
        other => other
    }
}














