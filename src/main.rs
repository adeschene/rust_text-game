//! # Text Adventure Game
//!
//! A text adventure game in the spirit of classic games like Zork, etc.

extern crate rustyline;

mod room;
mod state;

use rustyline::Editor;
use std::process;
use std::fs;

const INTRO_PATH: &str   = "../data/intro.txt";
const HISTORY_PATH: &str = "../data/history.txt";
const SAVE_PATH: &str    = "../data/savedgame.txt";


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
        print!("\n___Main_Menu___\n\n'new' for new game\n'continue' to load game\n'quit' to exit\n");
        let choice = rl.readline("\n> ").expect("Readline error");
        match choice.to_ascii_lowercase().as_str() {
            "new"      => { break },
            "continue" => { is_saved_game = true; break },
            "quit"     => { println!("\n\nExiting!"); process::exit(1) },
            _          => println!("\nInvalid choice!")
        }
    }
    // Set game state to new game state, change when loading.
    let mut gstate = state::State::new();
    // Prints different intro text depending on type of game.
    if is_saved_game {
        println!("Welcome back!");
        // Load input history from prior play sessions.
        // Returns error if there is no history file.
        rl.load_history(HISTORY_PATH)
          .expect("history.txt is missing or inaccessible");
        gstate = load_game(SAVE_PATH); // Load previous game state
    } else {
        print_intro_text(INTRO_PATH);
        save_game(gstate.serialize(), SAVE_PATH)
    }
    // Main game / user command loop.
    loop {
        let input = rl.readline("\n> ").expect("Readline error");
        rl.add_history_entry(input.as_ref());
        match input.to_ascii_lowercase().as_str() {
            "north" => { gstate.curr_room = room::go_north(gstate.curr_room); }
            "west"  => { gstate.curr_room = room::go_west(gstate.curr_room); }
            "south" => { gstate.curr_room = room::go_south(gstate.curr_room); }
            "east"  => { gstate.curr_room = room::go_east(gstate.curr_room); }
            "quit"  => {
                           println!("\n\nExiting!"); 
                           rl.save_history(HISTORY_PATH).unwrap();
                           save_game(gstate.serialize(), SAVE_PATH);
                           process::exit(1)
                       },
            other   => println!("\n{} is not a valid command.", other)
        }
        println!("\nRoom name: {:?}\n", room::get_info(gstate.curr_room));
    }
}


/// Prints the intro text contained in /data/intro.txt
/// and displays it to the user when they start a new game.
///
/// Does not display when a saved game is loaded.
///
/// If intro.txt is missing, returns a file read error.

pub fn print_intro_text(path: &str) {
    let intro = fs::read_to_string(path)
        .expect("intro.txt is missing or inaccessible");
    println!("{}",intro);
}

/// Uses a serialized version of the game state and a
/// save file path to save the current game.
///
/// Can return file write error.

pub fn save_game(gs: String, path: &str) {
    fs::write(path, &gs)
        .expect("Error writing save.txt file");
}

/// Deserializes the save file in the supplied path
/// and then populates the game state with the info
/// found, and returns that game state.

pub fn load_game(path: &str) -> state::State {
    state::State::deserialize(path)
}




















