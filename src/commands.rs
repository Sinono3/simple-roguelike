use std::io;
use std::io::prelude::*;
use crossterm::style::{Color, style};

use crate::game_state::{GameState, PLAYER_ID};
use crate::creatures::CreatureId;

pub fn pause() {
    // Read a single byte and discard
    let _ = io::stdin().read(&mut [1u8]).unwrap();
    let _ = io::stdin().read(&mut [1u8]).unwrap();
}

const DEBUG_MODE_ENABLED: bool = true;

pub enum Command {
	Attack(CreatureId),
	Examine(CreatureId),
	Status,
    Help,
    Debug(DebugCommand)
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
            // commands, such as 'status' or 'help'

			match parts[0] {
				"attack" => {
					if parts.len() > 1 {
						if let Some(target) = state.creatures.find_by_name(parts[1]) {
                            if target != PLAYER_ID {
                                break Command::Attack(target);
                            } else {
                                println!("{}", style("Don't attack yourself!")
                                            .with(Color::DarkRed))
                            }
						}
					}
                    println!("{}", style("Please write a correct target: ex: 'attack goblin'.")
                                   .with(Color::DarkRed))
				}
				"examine" => {
					if parts.len() > 1 {
						if let Some(target) = state.creatures.find_by_name(parts[1]) {
							break Command::Examine(target);
						}
					}
                    println!("{}", style("Please write a correct target: ex: 'examine goblin'.")
                                   .with(Color::DarkRed))
				}
				"status" => {
					break Command::Status;
				}
				"help" => {
    				break Command::Help;
				}
                "debug" => {
                    if DEBUG_MODE_ENABLED {
                        if parts.len() > 1 {
        					match parts[1] {
                                "remove" => {
                                    if parts.len() > 2 {
                                        if let Some(target) = state.creatures.find_by_name(parts[2]) {
                        				    break Command::Debug(DebugCommand::Remove(target));
                					    }
                                    }
                                }
                                _ => println!("{}",
                                              style(format!("'{}' is not a correct debug command.", parts[1]))
                                              .with(Color::DarkRed))
                            }
        				}
                        println!("{}", style("Please write an existing debug command: 'debug remove goblin'.")
                                       .with(Color::DarkRed))
                    } else {
                        println!("{}", style("Debug mode is disabled.")
                                   .with(Color::DarkRed))
                    }
                }
                _ => println!("{}",
                              style(format!("'{}' is not a correct command.", parts[0]))
                              .with(Color::DarkRed))
			}

			input_string_buffer.clear();
		}
	}
}
