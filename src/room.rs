
// Denotes the end-game room.
pub const FINAL_ROOM: usize = 15;
pub const CELL: usize = 0;
pub const SECRET_ROOM: usize = 16;

/// Based on usize room index, returns the name
/// and description of the room.
///
/// # Panics
///
/// Will panic if given a room that can't be matched,
/// as all rooms must be accounted for.

pub fn get_desc(room_index: usize, has_key: bool) -> String {
    match room_index {
        0  => "You are in a cramped cell.\n\
               The only door is straight ahead.\n\
               The wall directly behind you looks \
               off, somehow...".to_string(),
        1  => "Can go in any direction.".to_string(),
        2  => "Another short hallway.".to_string(),
        3  => "Can go east or west.".to_string(),
        4  => "Can go east or west.".to_string(),
        5  => "Can go east or west.".to_string(),
        6  => "Can go east, north, or south.".to_string(),
        7  => "Can go north or south.".to_string(),
        8  => "Can go east or north.".to_string(),
        9  => "Can go east or west.".to_string(),
        10 => "Can go east or west.".to_string(),
        11 => "Can go west.".to_string(),
        12 => "Can go east or west.".to_string(),
        13 => "Can go north or west.".to_string(),
        14 => "Can go south.".to_string(),
        15 => "Final room.".to_string(),
        16 => {
                  if has_key {
                      "You are in a small room, dimly lit.\n\
                      You see an empty table \
                      in the corner of the room.\n\
                      There doesn't seem to be anything of \
                      interest in here anymore.".to_string()
                  } else {
                      "You are in a small room, dimly lit.\n\
                      You catch the glint of metal on a table\n\
                      in the corner of the room.\n\
                      The room appears to be otherwise empty.".to_string()
                  }
              },
        _  => panic!() // We should be guaranteed an existing room.
    }
}

/// Move player to room immediately North of the
/// current room, or not, if there is no such room.

pub fn go_north(room_index: usize, has_key: bool) -> usize {
    match room_index {
        0    => {
                     if has_key {
                         println!("\n\nYou head through the door.\n\
                                  Freedom! Kind of...");
                         1
                     } else {
                         println!("\n\nThe door is locked tight.");
                         0
                     }
                 },
        1     => {
                     println!("\n\nYou enter the northern room.");
                     2
                 },
        2     => {
                     println!("\n\nYou enter the northern room.");
                     3
                 },
        6     => {
                     println!("\n\nYou enter the northern room.");
                     15 
                 },
        7     => {
                     println!("\n\nYou enter the northern room.");
                     6
                 },
        8     => {
                     println!("\n\nYou enter the northern room.");
                     7
                 },
        13    => {
                     println!("\n\nYou enter the northern room.");
                     14
                 },
        16    => {
                     println!("\n\nYou enter the northern room.");
                     0
                 },
        other => {
                     println!("\n\nYou can't go north from here.");
                     other
                 },
    }
}

/// Move player to room immediately West of the
/// current room, or not, if there is no such room.

pub fn go_west(room_index: usize) -> usize {
    match room_index {
        1     => {
                     println!("\n\nYou enter the western room.");
                     10
                 },
        3     => {
                     println!("\n\nYou enter the western room.");
                     4
                 },
        4     => {
                     println!("\n\nYou enter the western room.");
                     5
                 },
        5     => {
                     println!("\n\nYou enter the western room.");
                     6
                 },
        9     => {
                     println!("\n\nYou enter the western room.");
                     8
                 },
        10    => {
                     println!("\n\nYou enter the western room.");
                     9
                 },
        11    => {
                     println!("\n\nYou enter the western room.");
                     1
                 },
        12    => {
                     println!("\n\nYou enter the western room.");
                     3
                 },
        13    => {
                     println!("\n\nYou enter the western room.");
                     12
                 },
        other => {
                     println!("\n\nYou can't go west from here.");
                     other
                 },
    }
}

/// Move player to room immediately South of the
/// current room, or not, if there is no such room.

pub fn go_south(room_index: usize, found_room: bool) -> usize {
    match room_index {
        0     => {
                     if found_room {
                         println!("\n\nYou head through the opening and \
                                   into the strange, small room.");
                         16
                     } else {
                         println!("\n\nSomethings clearly off about the wall,\n\
                                   but you can't just walk through it.");
                         0
                     }
                 },
        1     => {
                     println!("\n\nYou enter the southern room.");
                     0
                 },
        2     => {
                     println!("\n\nYou enter the southern room.");
                     1
                 },
        6     => {
                     println!("\n\nYou enter the southern room.");
                     7
                 },
        7     => {
                     println!("\n\nYou enter the southern room.");
                     8
                 },
        14    => {
                     println!("\n\nYou enter the southern room.");
                     13
                 },
        15    => {
                     println!("\n\nYou enter the southern room.");
                     6
                 },
        other => {
                     println!("\n\nYou can't go south from here.");
                     other
                 },
    }
}

/// Move player to room immediately East of the
/// current room, or not, if there is no such room.

pub fn go_east(room_index: usize) -> usize {
    match room_index {
        1     => {
                     println!("\n\nYou enter the eastern room.");
                     11
                 },
        3     => {
                     println!("\n\nYou enter the eastern room.");
                     12
                 },
        4     => {
                     println!("\n\nYou enter the eastern room.");
                     3
                 },
        5     => {
                     println!("\n\nYou enter the eastern room.");
                     4
                 },
        6     => {
                     println!("\n\nYou enter the eastern room.");
                     5
                 },
        8     => {
                     println!("\n\nYou enter the eastern room.");
                     9
                 },
        9     => {
                     println!("\n\nYou enter the eastern room.");
                     10
                 },
        10    => {
                     println!("\n\nYou enter the eastern room.");
                     1
                 },
        12    => {
                     println!("\n\nYou enter the eastern room.");
                     13
                 },
        other => {
                     println!("\n\nYou can't go east from here.");
                     other
                 },
    }
}














