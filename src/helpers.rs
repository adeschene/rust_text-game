use std::fs;
use std::io;

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

/// Pauses the game loop execution after showing info
/// to the player, with the intention of letting the 
/// player examine the information and then press the
/// enter key to move on when they're ready to.

pub fn wait_for_player() {
    println!("\n\n > Press RETURN to continue <");
    let mut unused = String::new();
    let _ = io::stdin().read_line(&mut unused);
}
