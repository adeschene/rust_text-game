//! # Text Adventure Game
//!
//! A text adventure game in the spirit of classic games like Zork, etc.

extern crate rustyline;

mod room;
mod state;

use rustyline::Editor;
use std::process;
use std::fs;

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
    // Main menu loop
    loop {
        print_from_file(MENU_PATH);
        let choice = rl.readline("\n> ").expect("Readline error");
        match choice.to_ascii_lowercase().as_str() {
            "new"      => { break },
            "continue" => { is_saved_game = true; break },
            "quit"     => { println!("\n\nExiting!"); process::exit(1) },
            _          => println!("\nInvalid choice!")
        }
    }
    // Set game state to new game state, change later if loading.
    let mut gstate = state::State::empty();
    // Prints different intro text depending on type of game.
    if is_saved_game {
        println!("Welcome back!");
        // Load input history from prior play sessions.
        // Returns error if there is no history file.
        rl.load_history(HISTORY_PATH)
          .expect("history.txt is missing or inaccessible");
        gstate = load_game(); // Load previous game state
    } else {
        print_from_file(INTRO_PATH);
    }
    // Main game / user command loop.
    loop {
        let input = rl.readline("\n> ").expect("Readline error");
        rl.add_history_entry(input.as_ref());
        // Split up the words in the user's input; only using first two.
        let mut input_iter = input.as_str().split_whitespace();
        // Parse the user's command and the argument following it.
        let cmd = parse_input(input_iter.next());
        let arg = parse_input(input_iter.next());
        // Use input to execute the desired command as best we can.
        match cmd.as_str() {
            "go" | "move"   => gstate = go_cmd(gstate, arg.as_str()),
            "help" | "?"    => print_from_file(HELP_PATH),
            "quit" | "exit" =>
                {
                    println!("\n\nExiting!"); 
                    rl.save_history(HISTORY_PATH).unwrap();
                    save_game(gstate.serialize());
                    process::exit(1)
                },
            other           =>
                println!("\n{} is not a valid command.", other),
        }
        // Update the state based on user's actions.
        gstate = gstate.update_flags(room::FINAL_ROOM);
        // For testing:
        println!("\nRoom info: {:?}\nIn final room? {:?}\n"
                 , room::get_info(gstate.curr_room)
                 , gstate.in_final_room);
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
        "north" | "up" | "forward" => { 
                       if gstate.in_final_room {
                           player_win(); // Escaped; Went north from final room.
                           gstate // Just satisfies the return type
                       } else {
                           state::State::new( room::go_north(gstate.curr_room),
                                              gstate.in_final_room,
                                            )
                       }
                   }
        "west" | "left" => {
                       state::State::new( room::go_west(gstate.curr_room),
                                          gstate.in_final_room,
                                        )
                   }
        "south" | "down" | "back" => {
                       state::State::new( room::go_south(gstate.curr_room),
                                          gstate.in_final_room,
                                        )
                   }
        "east" | "right" => {
                       state::State::new( room::go_east(gstate.curr_room),
                                          gstate.in_final_room,
                                        )
                   }
        other   => {
                       println!("\n{} is not a valid direction.", other);
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
        .expect("Error writing save.txt file");
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
    save_game("0\nfalse".to_string());
    process::exit(1);
}


















