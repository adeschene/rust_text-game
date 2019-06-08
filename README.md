
# Rust Text Game

Copyright (c) 2019 Alec Deschene

A small, experimental Text-Adventure Game created for Bart Massey's Spring 2019 Rust Programming course. 

Allows the player to explore and attempt to escape a small dungeon map.

## Instructions

- Install Cargo, if necessary.

- Clone repository:

git clone https://github.com/adeschene/rust_text-game.git

- Build game:

cargo build

- Run game:

cargo run

**NOTE**: Currently, the game will only work properly if the user's current working directory is '~/rust_text-game/src/'. This issue is currently being sorted out.

## How to play

Playability is currently very low. There are, however, a few things a player can do at the moment.

- The first menu presents the player with options to start a new game, continue a previous game, or quit. Enter 'new', 'continue', or 'quit' into the console.
- If either of the first two options was chosen in the last menu, a game will now have been started, and entering commands into the game prompt will cause different things to happen.
- Enter 'go' followed by a direction ('north', 'up', etc) to move to the adjacent room in that direction (if there is one.)
- Enter 'look' followed by an object to examine that object more closely / interact with it.
- Enter 'help' to display the help menu.
- Enter 'talk' to talk to a person who is in a room with you.
- Enter 'quit' to end the game and exit the program. This will save the game before exiting. If, upon starting the game again, one chooses 'continue' at the main menu prompt, the state of the game at the last exit will be loaded. In contrast, choosing 'new' will erase the previous game and start over from the beginning.
- For convenience, the up and down arrows will allow the player to move through previously entered inputs. This history is also saved, loaded, and overwritten the same way that the game state is.
- If the player navigates to the 'final room' and then exits through the northern door of that room, the game will be won, everything will reset, and the program/game will end.

This is currently the extent of the in-game functionality.

## Goals for this project
Implementation plans -- some more realistic than others -- in no particular order.
- Add more items
- Abstract out more main() functionality to helper functions
- Add more NPCs
- Add enemies and combat
- Add more storytelling elements
- Add more unit tests

## Reflections
Throughout this project, I've learned a lot about Rust, predictably. The borrow checker and I are now very well acquainted.
I feel like I understand a lot more about how a rust program is set up (module structures, etc.) than I did before.
Doc comments were a fun thing to figure out, and are really quite nice.
And of course, this project has been another in a long list of lessons on managing time and priorities.
Overall, it's been a lot of fun so far.

## Writeup
In addition to the information in this README, a more in depth analysis and writup can be found by looking at the file Rust_Text_Game_WriteUp.pdf.

## Author information
Alec Deschene: deschene@pdx.edu

## License
This project is put forth under the MIT License. More information can be found by examining the LICENSE file.
