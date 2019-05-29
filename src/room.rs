
/// Based on usize room index, returns the name
/// and description of the room.
///
/// # Panics
///
/// Will panic if given a room that can't be matched,
/// as all rooms must be accounted for.

pub fn get_info(room_index: usize) -> (String, String) {
    match room_index {
        0 => ("Cell".to_string(),
              "An empty cell with a single door.".to_string()),
        1 => ("Hallway".to_string(),
              "A short hallway.".to_string()),
        _ => panic!() // We should be guaranteed an existing room.
    }
}

/// Move player to room immediately North of the
/// current room, or not, if there is no such room.

pub fn go_north(room_index: usize) -> usize {
    match room_index {
        0     => 1,
        other => other
    }
}

/// Move player to room immediately West of the
/// current room, or not, if there is no such room.

pub fn go_west(room_index: usize) -> usize {
    match room_index {
        other => other
    }
}

/// Move player to room immediately South of the
/// current room, or not, if there is no such room.

pub fn go_south(room_index: usize) -> usize {
    match room_index {
        1     => 0,
        other => other
    }
}

/// Move player to room immediately East of the
/// current room, or not, if there is no such room.

pub fn go_east(room_index: usize) -> usize {
    match room_index {
        other => other
    }
}














