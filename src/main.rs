extern crate rustyline;

mod room;

use rustyline::Editor;
use std::process;
use std::fs;

pub fn main() {
    let mut rl = Editor::<()>::new();
    rl.load_history("../data/history.txt")
      .expect("history.txt is missing or inaccessible");
    let mut current_room = room::make_room("R00");
    let mut is_saved_game = false;
    loop {
        print!("\n___Main_Menu___\n\n'new' for new game\n'quit' to exit\n");
        let choice = rl.readline("\n> ").expect("Readline error");
        match choice.to_ascii_lowercase().as_str() {
            "new"      => { break },
            "continue" => { is_saved_game = true; break },
            "quit"     => { println!("\n\nExiting!"); process::exit(1) },
            _          => println!("\nInvalid choice!")
        }
    }
    if is_saved_game {
        println!("Welcome back!");
        //load_game()
    } else {
        print_intro_text()
        //save_game()
    }
    loop {
        let input = rl.readline("\n> ").expect("Readline error");
        rl.add_history_entry(input.as_ref());
        match input.to_ascii_lowercase().as_str() {
            "quit" => { println!("\n\nExiting!"); process::exit(1) },
            other  => println!("\n{} is not a valid command.", other)
        }
        println!("\nRoom name: {}\n", current_room.name);
    }
    //rl.save_history("../data/history.txt").unwrap();
}


// --- Helper Functions ---

pub fn print_intro_text() {
    let intro = fs::read_to_string("../data/intro.txt")
        .expect("intro.txt is missing or inaccessible");
    println!("\n__OBLIVION__\n\n{}",intro);
}


// --- Structures ---
























