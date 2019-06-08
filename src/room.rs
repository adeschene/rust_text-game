
use crate::helpers::print_from_file;

// Constants that denote various essential room numbers.
pub const FINAL_ROOM: usize = 7;
pub const CELL: usize = 0;
pub const SECRET_ROOM: usize = 12;
pub const BROOM_ROOM: usize = 11;
pub const SMELLY_CELL: usize = 8;

// Room text data file path constants
const R0_TEXT_0: &str  = "../data/room/r0text0.txt";
const R0_TEXT_1: &str  = "../data/room/r0text1.txt";
const R1_TEXT_0: &str  = "../data/room/r1text0.txt";
const R2_TEXT_0: &str  = "../data/room/r2text0.txt";
const R3_TEXT_0: &str  = "../data/room/r3text0.txt";
const R4_TEXT_0: &str  = "../data/room/r4text0.txt";
const R5_TEXT_0: &str  = "../data/room/r5text0.txt";
const R6_TEXT_0: &str  = "../data/room/r6text0.txt";
const R7_TEXT_0: &str  = "../data/room/r7text0.txt";
const R8_TEXT_0: &str  = "../data/room/r8text0.txt";
const R8_TEXT_1: &str  = "../data/room/r8text1.txt";
const R9_TEXT_0: &str  = "../data/room/r9text0.txt";
const R10_TEXT_0: &str = "../data/room/r10text0.txt";
const R11_TEXT_0: &str = "../data/room/r11text0.txt";
const R11_TEXT_1: &str = "../data/room/r11text1.txt";
const R12_TEXT_0: &str = "../data/room/r12text0.txt";
const R12_TEXT_1: &str = "../data/room/r12text1.txt";

/// Based on usize room index, prints a specific room's description.
/// Some rooms have alternate text displayed based on game state.
///
/// # Panics
///
/// Will panic if given a room that can't be matched,
/// as all rooms are expected to be accounted for.

pub fn get_desc(room_index: usize, has_key: bool,
                found_room: bool, has_broom: bool,
                has_nail: bool) {
    match room_index {
        0  => {
                  if found_room == false { print_from_file(R0_TEXT_0) }
                  else { print_from_file(R0_TEXT_1) }
              },
        1  => print_from_file(R1_TEXT_0),
        2  => print_from_file(R2_TEXT_0),
        3  => print_from_file(R3_TEXT_0),
        4  => print_from_file(R4_TEXT_0),
        5  => print_from_file(R5_TEXT_0),
        6  => print_from_file(R6_TEXT_0),
        7  => print_from_file(R7_TEXT_0),
        8  => {
                  if has_nail == false { print_from_file(R8_TEXT_0) }
                  else { print_from_file(R8_TEXT_1) }
              },
        9  => print_from_file(R9_TEXT_0),
        10 => print_from_file(R10_TEXT_0),
        11 => {
                  if has_broom == false { print_from_file(R11_TEXT_0) }
                  else { print_from_file(R11_TEXT_1) }
              },
        12 => {
                  if has_key == false { print_from_file(R12_TEXT_0) }
                  else { print_from_file(R12_TEXT_1) }
              },
        _  => panic!() // We should be guaranteed a valid room number.
    }
}

/// Move player to room immediately North of the
/// current room, or not, if there is no such room.

pub fn go_north(room_index: usize, has_key: bool,
                final_room_unlocked: bool) -> usize {
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
                 }
        1     => {
                     println!("\n\nYou sally forth!");
                     2
                 }
        2     => {
                     println!("\n\nYou speedwalk out of \
                               the old man's cold stare.");
                     3
                 }
        6     => {
                     if final_room_unlocked {
                         println!("\n\nWith the door now unlocked, \
                                   you head through, filled with anxiety.");
                         7
                     } else {
                         println!("\n\nThere's an entire human-\
                                   being in your way, unfortunately.");
                         6
                     }
                 }
        10    => {
                     println!("\n\nYou enter the northern room.");
                     11
                 }
        12    => {
                     println!("\n\nYou enter the northern room.");
                     0
                 }
        other => {
                     println!("\n\nYou can't go north from here.");
                     other
                 }
    }
}

/// Move player to room immediately West of the
/// current room, or not, if there is no such room.

pub fn go_west(room_index: usize) -> usize {
    match room_index {
        3     => {
                     println!("\n\nYou enter the western room.");
                     4
                 }
        4     => {
                     println!("\n\nYou enter the western room.");
                     5
                 }
        5     => {
                     println!("\n\nYou enter the western room.");
                     6
                 }
        8     => {
                     println!("\n\nYou enter the western room.");
                     1
                 }
        9     => {
                     println!("\n\nYou enter the western room.");
                     3
                 }
        10    => {
                     println!("\n\nYou enter the western room.");
                     9
                 }
        other => {
                     println!("\n\nYou can't go west from here.");
                     other
                 }
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
                         12
                     } else {
                         println!("\n\nSomethings clearly off about the wall,\n\
                                   but you can't just walk through it.");
                         0
                     }
                 }
        1     => {
                     println!("\n\nYou nervously head back into your cell...");
                     0
                 }
        2     => {
                     println!("\n\nYou speedwalk out of the old man's cold stare.");
                     1
                 }
        3     => {
                     println!("\n\nYou speedwalk out of the old man's cold stare.");
                     2
                 }
        7     => {
                     println!("\n\nYou enter the southern room.");
                     6
                 }
        11    => {
                     println!("\n\nYou enter the southern room.");
                     10
                 }
        other => {
                     println!("\n\nYou can't go south from here.");
                     other
                 }
    }
}

/// Move player to room immediately East of the
/// current room, or not, if there is no such room.

pub fn go_east(room_index: usize, met_blimpo: bool) -> usize {
    match room_index {
        1     => {
                     if met_blimpo {
                         println!("\n\nYou ready yourself, \
                                   take a deep breath, and \
                                   head into the stench.");
                         8
                     } else {
                         println!("\n\nThe stench radiating from the cell \
                                   is unbearable.\nNothing inside looks \
                                   interesting enough to justify going in.");
                         1
                     }
                 }
        3     => {
                     println!("\n\nYou enter the eastern room.");
                     9
                 }
        4     => {
                     println!("\n\nYou enter the eastern room.");
                     3
                 }
        5     => {
                     println!("\n\nYou enter the eastern room.");
                     4
                 }
        6     => {
                     println!("\n\nYou enter the eastern room.");
                     5
                 }
        9     => {
                     println!("\n\nYou enter the eastern room.");
                     10
                 }
        other => {
                     println!("\n\nYou can't go east from here.");
                     other
                 }
    }
}














