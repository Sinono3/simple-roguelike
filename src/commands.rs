use std::io;
use std::io::prelude::*;

use crate::features::GameState;
use crate::creatures::*;

pub fn pause() {
    // Read a single byte and discard
    let _ = io::stdin().read(&mut [1u8]).unwrap();
    let _ = io::stdin().read(&mut [1u8]).unwrap();
}

pub enum Command {
	Attack(CreatureId),
	Examine(CreatureId)
}

impl Command {
	pub fn get(state: &GameState) -> Command {
		let stdin = io::stdin();
		let mut buffer = String::new();
		
		loop {
			stdin.read_line(&mut buffer).unwrap();
			
			let parts: Vec<&str> = buffer.trim().split(' ').collect();
			
			match parts[0] {
				"attack" => {
					if parts.len() > 1 {
						if let Some(target) = state.find_creature(parts[1]) {
							break Command::Attack(target);
						}
					}
					println!("Please write a correct target: 'attack goblin1'");
				}
				"examine" => {
					if parts.len() > 1 {
						if let Some(target) = state.find_creature(parts[1]) {
							break Command::Examine(target);
						}
					}
					println!("Please write a correct target: 'examine goblin1'");
				}
				_ => {
					println!("'{}' is not a correct command.", parts[0]);
				}
			}
			
			buffer.clear();
		}
	}
}