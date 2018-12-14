use std::io;
use std::io::prelude::*;

use crate::features::GameState;
use crate::creatures::*;

pub fn pause() {
    // Read a single byte and discard
    let _ = io::stdin().read(&mut [1u8]).unwrap();
    let _ = io::stdin().read(&mut [1u8]).unwrap();
}

const DEBUG_MODE_ENABLED: bool = true;

pub enum Command {
	Attack(CreatureId),
	Examine(CreatureId),
    Debug(DebugCommand),
	Status(),
	Error()
}
pub enum DebugCommand {
    Remove(CreatureId)
}

impl Command {
	pub fn get(state: &GameState) -> Command {
		let stdin = io::stdin();
		let mut input_string_buffer = String::new();
		

		loop {
			
			stdin.read_line(&mut input_string_buffer).unwrap();
			
			
			let parts: Vec<&str> = input_string_buffer.trim().split(' ').collect();

            // The repetition of parts.len() > 1 is acknowledged but is necessary due to one-worded
            // commands, which will be implemented in later versions of the engine/game.
		
			match parts[0] {
				"attack" => {
					if parts.len() > 1 {
						if let Some(target) = state.creatures.find(parts[1]) {
							break Command::Attack(target);
						}
					}
					println!("Please write a correct target: eg: 'attack goblin'.");
				}
				"examine" => {
					if parts.len() > 1 {
						if let Some(target) = state.creatures.find(parts[1]) {
							break Command::Examine(target);
						}
					}
					println!("Please write a correct target: eg: 'examine goblin'.");
				}
				"status" => {
					break Command::Status();
				}
				"help" => {
					println!("For play you have this commands: \n
	attack: Attack to enemies\n
		Usage=> attack name_of_enemy\n
	examine\n
	status: Show the actual game status\n"								
					);
					
				}
                "debug" => {
                    if DEBUG_MODE_ENABLED {
                        if parts.len() > 1 {
        					match parts[1] {
                                "remove" => {
                                    if parts.len() > 2 {
                                        if let Some(target) = state.creatures.find(parts[2]) {
                        				    break Command::Debug(DebugCommand::Remove(target));
                					    }
                                    }
                                }
                                _ => println!("'{}' is not a correct debug command.", parts[1])
                            }
        				}
        				println!("Please write an existing debug command: 'debug remove goblin'.");
                    } else {
                        println!("Debug mode is disabled.");
                    }
                }
				_ => {
					println!("'{}' is not a correct command.", parts[0]);
					break Command::Error();
				}
			}

			input_string_buffer.clear();
		}
	}
}
