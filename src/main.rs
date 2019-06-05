//! # Text Adventure Game
//!
//! A text adventure game in the spirit of classic games like Zork, etc.

extern crate rustyline;

mod room;
mod state;

use rustyline::Editor;
use std::process;
use std::fs;
use std::io;

// Filename constants; TODO: File path expansion
const INTRO_PATH: &str   = "../data/intro.txt";
const HISTORY_PATH: &str = "../data/history.txt";
const SAVE_PATH: &str    = "../data/savedgame.txt";
const HELP_PATH: &str    = "../data/help.txt";
const MENU_PATH: &str    = "../data/mainmenu.txt";


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
    print_from_file(INTRO_PATH);
    // Main menu loop
    loop {
        print_from_file(MENU_PATH);
        let choice = rl.readline("\n> ").expect("Readline error");
        match choice.to_ascii_lowercase().as_str() {
            "new" | "n"      => { break },
            "continue" | "c" => { is_saved_game = true; break },
            "quit" | "q"     => { println!("\n\nExiting!"); process::exit(1) },
            _                => println!("\nInvalid choice!")
        }
    }
    // Set game state to new game state, change later if loading.
    let mut gstate = state::State::empty();
    // Load last gamestate and input history if "continuing".
    if is_saved_game {
        println!("\n\nWelcome back!");
        // Load input history from prior play sessions.
        // Returns error if there is no history file.
        rl.load_history(HISTORY_PATH)
          .expect("history file is missing or inaccessible");
        gstate = load_game(); // Load previous game state
    }
    // Main game / user command loop.
    loop {
        // Display the room description to the player.
        println!("\n\n{}", room::get_desc(gstate.curr_room, gstate.took_key));
        let input = rl.readline("\n> ").expect("Readline error");
        rl.add_history_entry(input.as_ref());
        // Split up the words in the user's input; only using first two.
        let mut input_iter = input.as_str().split_whitespace();
        // Parse the user's command and the argument following it.
        let cmd = parse_input(input_iter.next());
        let arg = parse_input(input_iter.next());
        // Use input to execute the desired command as best we can.
        match cmd.as_str() {
            "go" | "move" | "walk" =>
                {
                    gstate = go_cmd(gstate, arg.as_str());
                    wait_for_player();
                },
            "look" | "examine"     => 
                {
                    gstate = look_cmd(gstate, arg.as_str());
                    wait_for_player();
                },
            "help" | "?"           =>
                {
                    print_from_file(HELP_PATH);
                    wait_for_player();
                },
            "quit" | "exit" | "q"  =>
                {
                    println!("\n\nExiting!"); 
                    rl.save_history(HISTORY_PATH).unwrap();
                    save_game(gstate.serialize());
                    process::exit(1)
                },
            other                  =>
                {
                    println!("\n{} is not a valid command.", other);
                    wait_for_player();
                },
        }
        // Update the state based on user's actions.
        gstate = gstate.update_flags(room::FINAL_ROOM);
    }
}

/// Allows the player to move from one room to a room directly
/// adjacent to it. Uses directional commands and some alternatives
/// to decide which direction the player wants to go.
///
/// Returns a new game state with the updated current_room.
/// If player can't go in the desired direction, returns old state.

pub fn go_cmd(gstate: state::State, dir: &str) -> (state::State) {
    match dir {
        "north" | "up" | "forward" =>
            { 
                if gstate.in_final_room {
                    player_win(); // Escaped; Went north from final room.
                    gstate // Just satisfies the return type
                } else {
                    state::State::new
                        (   room::go_north(gstate.curr_room,
                                           gstate.took_key),
                            gstate.in_final_room,
                            gstate.examined_wall,
                            gstate.took_key,
                        )
                }
            }
        "west" | "left"            =>
            {
                state::State::new
                    (   room::go_west(gstate.curr_room),
                        gstate.in_final_room,
                        gstate.examined_wall,
                        gstate.took_key,
                    )
            }
        "south" | "down" | "back"  =>
            {
                state::State::new
                    (   room::go_south(gstate.curr_room,
                                       gstate.examined_wall),
                        gstate.in_final_room,
                        gstate.examined_wall,
                        gstate.took_key,
                    )
            }
        "east" | "right"           =>
            {
                state::State::new
                    (   room::go_east(gstate.curr_room),
                        gstate.in_final_room,
                        gstate.examined_wall,
                        gstate.took_key,
                    )
            }
        other                      =>
            {
                println!("\n{} is not a valid direction.", other);
                gstate
            }
    }
}

/// 
/// 
/// Returns a new game state with the updated player discoveries.
/// If the desired object is not examinable, returns old state.

pub fn look_cmd(gstate: state::State, obj: &str) -> (state::State) {
    match obj {
        "wall"  =>
            { 
                if gstate.curr_room == room::CELL {
                    if gstate.examined_wall {
                        println!("\n\nYou see the entryway to the secret room.");
                        gstate
                    } else {
                        println!(
                            "\n\nYou see a slight indentation in the wall.\n\
                            You put you hand against it and push gently.\n\
                            As soon as you apply the least bit of pressure,\n\
                            the indentation pushes inward, revealing\n\
                            a small, dimly lit room."
                                );
                        state::State::new
                            (   gstate.curr_room,
                                gstate.in_final_room,
                                true,
                                gstate.took_key,
                            )
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
                        state::State::new
                            (   gstate.curr_room,
                                gstate.in_final_room,
                                gstate.examined_wall,
                                true,
                            )
                    }
                } else {
                    println!("\n\nHmmm...there aren't \
                              any tables around you...");
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

/// Takes input str that may or may not exist,
/// determines whether it does, and then returns
/// the input str converted to lowercase or an
/// empty string.

pub fn parse_input(input: Option<&str>) -> String {
    match input {
        Some(s) => s.to_ascii_lowercase(),
        None    => "".to_string()
    }
}

/// Prints text contained in a file and displays it
/// to the user. 
///
/// If the supplied path is invalid,
/// causes a file read error.

pub fn print_from_file(path: &str) {
    let file_text = fs::read_to_string(path).unwrap();
    println!("{}", file_text);
}

/// Uses a serialized version of the game state and a
/// save file path to save the current game.
///
/// Can return file write error.

pub fn save_game(gs: String) {
    fs::write(SAVE_PATH, &gs)
        .expect("Error writing save file");
}

/// Deserializes the save file in the supplied path
/// and then populates the game state with the info
/// found, and returns that game state.

pub fn load_game() -> state::State {
    state::State::deserialize(SAVE_PATH)
}

/// When the player leaves through the northern door of
/// the final room, the player wins the game.
///
/// Displays a congratulatory message and then erases the
/// player's saved game and exits the program/game.
/// Doesn't touch user input history.

pub fn player_win() {
    println!("\n\nCongratulations!\n\nYou escaped!\n");
    wait_for_player();
    save_game("0\nfalse".to_string());
    process::exit(1);
}

/// Pauses the game loop execution after showing info
/// to the player, with the intention of letting the 
/// player examine the information and then press the
/// enter key to move on when they're ready to.

pub fn wait_for_player() {
    println!("\n\n > Press RETURN to continue <");
    let mut unused = String::new();
    let _ = io::stdin().read_line(&mut unused);
}
















