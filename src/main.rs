//! # Text Adventure Game
//!
//! A text adventure game in the spirit of classic games like Zork, etc.

extern crate rustyline;

mod room;
mod state;
mod npcs;
mod helpers;

use rustyline::Editor;
use std::process;
use std::fs;

// Filename constants; TODO: File path expansion
const INTRO_PATH: &str   = "../data/misc/intro.txt";
const HISTORY_PATH: &str = "../data/misc/history.txt";
const SAVE_PATH: &str    = "../data/misc/savedgame.txt";
const HELP_PATH: &str    = "../data/misc/help.txt";
const MENU_PATH: &str    = "../data/misc/mainmenu.txt";
const NPCS_DEF: &str     = "../data/npc/npcsdefault.txt";
const STATE_DEF: &str    = "../data/state/statedefault.txt";


/// Contains the main game loop, the main menu loop,
/// makes the rustyline Editor, loads rustyline history,
/// loads and saves games, among other things.
///
/// Many things will ultimately be abstracted out to helpers.

pub fn main() {
    // rustyline structure, allows us to read input w/prompt.
    let mut rl = Editor::<()>::new();
    // Keep track of whether saved game or new game.
    let mut is_saved_game = false;
    // Print game intro text.
    helpers::print_from_file(INTRO_PATH);
    // Main menu loop
    loop {
        helpers::print_from_file(MENU_PATH); // Print main menu text
        let choice = rl.readline("\n> ").expect("Readline error");
        match choice.to_ascii_lowercase().as_str() {
            "new" | "n"      => { break },
            "continue" | "c" => { is_saved_game = true; break },
            "quit" | "q"     => { println!("\n\nExiting!"); process::exit(1) },
            _                => println!("\nInvalid choice!")
        }
    }
    // Load previous input history if "continuing".
    if is_saved_game {
        println!("\n\nWelcome back!");
        // Load input history from prior play sessions.
        rl.load_history(HISTORY_PATH)
          .expect("history file is missing or inaccessible");
    }
    // Set a new game state or load the saved one.
    // Do the same with the npcs and their states.
    let (mut gstate, mut npcs) = {
        if is_saved_game { load_game() }
        else { (state::State::empty(), npcs::generate_npcs()) }
    };
    // Main game / user command loop.
    loop {
        // Display the current room description to the player.
        room::get_desc(gstate.curr_room, gstate.took_key,
                       gstate.examined_wall, gstate.took_broom,
                       gstate.took_nail);
        // Prompt for user input: command [argument]
        let input = rl.readline("\n> ").expect("Readline error");
        // Add this input to user input history.
        rl.add_history_entry(input.as_ref());
        // Split up the words in the user's input; only using first two.
        let mut input_iter = input.as_str().split_whitespace();
        // Parse the user's command and the argument following it.
        let cmd = helpers::parse_input(input_iter.next());
        let arg = helpers::parse_input(input_iter.next());
        // Use input to execute the desired command as best we can.
        match cmd.as_str() {
            "go" | "move" | "walk" =>
                {
                    gstate = go_cmd(gstate, arg.as_str());
                    helpers::wait_for_player();
                },
            "look" | "examine"     => 
                {
                    gstate = look_cmd(gstate, arg.as_str());
                    helpers::wait_for_player();
                },
            "talk" | "speak" | "t" => 
                {
                    let temp = talk_cmd(npcs, gstate);
                    npcs = temp.0;
                    gstate = temp.1;
                    helpers::wait_for_player();
                },
            "help" | "?"           =>
                {
                    helpers::print_from_file(HELP_PATH);
                    helpers::wait_for_player();
                },
            "quit" | "exit" | "q"  =>
                {
                    println!("\n\nExiting!"); 
                    rl.save_history(HISTORY_PATH).unwrap();
                    save_game(gstate.serialize(), npcs::serialize(npcs));
                    process::exit(1)
                },
            other                  =>
                {
                    println!("\n{} is not a valid command.", other);
                    helpers::wait_for_player();
                },
        }
    }
}

/// Allows the player to move from one room to a room directly
/// adjacent to it. Uses directional commands and some alternatives
/// to decide which direction the player wants to go.
///
/// Returns a new game state with the updated current_room.
/// If player can't go in the desired direction, returns old state.

pub fn go_cmd(gstate: state::State, dir: &str) -> state::State {
    state::State::new(
        match dir {
            "north" | "up" | "forward" =>
                { 
                    if gstate.curr_room == room::FINAL_ROOM {
                        player_win(); // Escaped; Went north from final room.
                        gstate.curr_room // Just satisfies the return type
                    } else {
                        room::go_north(gstate.curr_room,
                                       gstate.took_key,
                                       gstate.final_room_unlocked)
                    }
                }
            "west" | "left"            =>
                room::go_west(gstate.curr_room),
            "south" | "down" | "back"  =>
                room::go_south(gstate.curr_room, gstate.examined_wall),
            "east" | "right"           =>
                room::go_east(gstate.curr_room, gstate.met_blimpo),
            other                      =>
            {
                println!("\n{} is not a valid direction.", other);
                gstate.curr_room
            }
        },
        gstate.examined_wall,
        gstate.took_key,
        gstate.took_broom,
        gstate.took_nail,
        gstate.met_blimpo,
        gstate.final_room_unlocked,
    )
}

#[test]
fn go_cmd_test() {
    let start1 = state::State::new(2,false,false,false,false,false,false);
    let start2 = state::State::new(2,false,false,false,false,false,false);
    // Going up from room 2 leads to room 3, so this should be true
    assert_eq!(3, go_cmd(start1, "up").curr_room);
    // Going up from room 3 is not possible, so this should be true
    assert_eq!(3, go_cmd(go_cmd(start2, "up"), "up").curr_room);
}

/// Allows player to closely examine key objects and effectively "take"
/// key items when they are look at.
/// 
/// Returns a new game state with the updated player discoveries.
/// If the desired object is not examinable, returns old state.

pub fn look_cmd(gstate: state::State, obj: &str) -> state::State {
    match obj {
        "wall"  =>
            {   // Only examine wall if player is in the right room.
                if gstate.curr_room == room::CELL {
                    // If player already looked at wall,
                    // display alternate description.
                    if gstate.examined_wall {
                        println!("\n\nYou see the entryway \
                                  to the secret room.");
                        gstate
                    } else {
                        println!(
                            "\n\nYou see a slight indentation in the wall.\n\
                            You put you hand against it and push gently.\n\
                            As soon as you apply the least bit of pressure,\n\
                            the indentation pushes inward, revealing\n\
                            a small, dimly lit room."
                                );
                            // set gstate.examined_wall to true
                            gstate.update((1, true))
                    }
                } else {
                    println!("\n\nYou look carefully at each wall...\
                              for some reason.");
                    gstate
                }
            }
        "table" =>
            { 
                if gstate.curr_room == room::SECRET_ROOM {
                    if gstate.took_key {
                        println!("\n\nThe table is now empty...");
                        gstate
                    } else {
                        println!("\n\nYou walk up to the table and look \
                                  at it more closely.\nOn the table \
                                  there is a key and nothing else.\n\
                                  You take the key with trembling fingers.");
                        // set gstate.took_key to true
                        gstate.update((2, true))
                    }
                } else {
                    println!("\n\nHmmm...there aren't \
                              any tables around you...");
                    gstate
                }
            }
        "broom" =>
            { 
                if gstate.curr_room == room::BROOM_ROOM {
                    if gstate.took_broom {
                        println!("\n\nIt's a broom...");
                        gstate
                    } else {
                        println!("\n\nYou walk closer to the broom and \
                                  take a closer look at it.\n\
                                  Ironically, it's filthy.\n\
                                  Despite this, you take the broom.");
                        // set gstate.took_broom to true
                        gstate.update((3, true))
                    }
                } else {
                    println!("\n\nThere are no brooms in sight...");
                    gstate
                }
            }
        "nail"  =>
            { 
                if gstate.curr_room == room::SMELLY_CELL {
                    if gstate.took_nail {
                        println!("\n\nIt's a bent, jagged nail.");
                        gstate
                    } else {
                        println!("\n\nYou approach the cot.\n\
                                  The closer you get, \
                                  the stronger the scent.\n\
                                  You quickly grab the nail, \
                                  without touching the cot itself,\n\
                                  and move swiftly away from the bed.");
                        // set gstate.took_nail to true
                        gstate.update((4, true))
                    }
                } else {
                    println!("\n\nThere are no brooms in sight...");
                    gstate
                }
            }
        "self"  =>
            {
                println!("\n\nThat's a little vain, isn't it?");
                gstate
            }
        other   =>
            {
                println!("\n\nCan't examine {}.", other);
                gstate
            }
    }
}

#[test]
fn look_cmd_test_ok() {
    let start = state::State::new(0,false,false,false,false,false,false);
    // Looking at wall in room 0 should set examined_wall to true,
    // so this should be true
    assert!(look_cmd(start, "wall").examined_wall);
}

#[test]
#[should_panic]
fn look_cmd_test_fail() {
    let start = state::State::new(1,false,false,false,false,false,false);
    // Looking at wall in room 1 should set not affect anything,
    // so this should be false and cause panic
    assert!(look_cmd(start, "wall").examined_wall);
}

/// Allows player to communicate with whatever NPC is in the same room.
/// Currently, we're guaranteed to only have one NPC in any given room.
///
/// Takes a pair containing a vec of NPCs and a gamestate,
/// updates them if necessary, and the returns the same pair type.

pub fn talk_cmd(mut npcs: Vec<npcs::Npc>,
                gstate: state::State) -> (Vec<npcs::Npc>, state::State) {
    // Case for NPC named Carl
    if gstate.curr_room == npcs[0].location {
        if gstate.took_broom && npcs[0].given_quest_item == false {
            npcs[0] = npcs[0].receive_item().speak();
        } else {
            npcs[0] = npcs[0].speak();
        }
    // Case for NPC named Blimpo
    } else if gstate.curr_room == npcs[1].location {
        if gstate.took_nail && npcs[1].given_quest_item == false {
            npcs[1] = npcs[1].receive_item().speak();
        } else if npcs[1].quest_done {
            npcs[1] = npcs[1].speak();
            // set gstate.final_room_unlocked to true
            return (npcs, gstate.update((6, true)))
        } else {
            npcs[1] = npcs[1].speak();
            // set gstate.met_blimpo to true
            return (npcs, gstate.update((5, true)))
        }
    // Case for room without an NPC present
    } else { println!("\n\nThere's nobody to talk to..."); }
    (npcs, gstate)
}

// TODO: Unit tests for talk_cmd

/// Uses a serialized version of the game state,
/// combined with a serialized version of the NPC
/// vec to save the current game to a file.
///
/// Can return file write error.

pub fn save_game(gs: String, npcs: String) {
    let mut data = gs;
    data.push_str(&npcs);
    fs::write(SAVE_PATH, &data).expect("Error writing save file");
}

/// Deserializes the save file in the supplied path
/// and then populates the game state and NPC vec with
/// the info found, and returns a pair type (Vec<Npc>, State).

pub fn load_game() -> (state::State, Vec<npcs::Npc>) {
    let data = fs::read_to_string(SAVE_PATH).unwrap();
    // Split on '~' separates State from NPC info
    let split_data: Vec<&str> = data.as_str().split("~").collect();
    ( state::State::deserialize(split_data[0]),
      npcs::deserialize(split_data[1]) )
}

/// When the player leaves through the northern door of
/// the final room, the player wins the game.
///
/// Displays a congratulatory message and then erases the
/// player's save data and exits the program/game.
/// Doesn't touch user input history.

pub fn player_win() {
    println!("\n\nCongratulations!\n\nYou made it!\n");
    helpers::wait_for_player();
    let npcs_default = fs::read_to_string(NPCS_DEF).unwrap();
    let state_default = fs::read_to_string(STATE_DEF).unwrap();
    // Save a default game for safety when player chooses 'continue' at menu.
    save_game(state_default, npcs_default);
    process::exit(1);
}












