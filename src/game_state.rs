use crossterm::style::{Color, style};

use crate::components::*;
use crate::creatures::*;
use crate::commands::{Command, DebugCommand};

pub const PLAYER_ID: CreatureId = 0;

pub struct GameState {
	pub creatures: CreatureMap
}

// just used for determining console output
enum AttackDirection {
	ToPlayer,
	Neutral,
	FromPlayer
}
impl AttackDirection {
	fn to_color(&self) -> Color {
		match self {
			AttackDirection::ToPlayer => Color::Red,
			AttackDirection::Neutral => Color::White,
			AttackDirection::FromPlayer => Color::Green
		}
	}
}
impl GameState {
	pub fn new(player: CreatureData) -> GameState {
		let mut state = GameState {
			creatures: CreatureMap::new()
		};
		state.creatures.add(player);
		return state;
	}
	pub fn round(&mut self) -> bool {
		// systems.
		player_system(self);
		crate::components::systems::aggression(self);

		true // TODO: player_system can return this, if not then the game will close because of the player's will
	}
	// Hits a creature with the inflictor's name and damage.
	pub fn hit(&mut self, inflictor_id: CreatureId, target_id: CreatureId) {
		assert!(inflictor_id != target_id, "Game logic error: a creature can't attack itself.");

		// get name and damage from inflictor
		let (name, damage) = {
			// Can use unwrap because the target the inflictor is hitting must exist
			(self.creatures.get::<NameComponent>(inflictor_id)
					.expect("Game logic error: Inflictor doesn't have a name.").0.clone(),
			self.creatures.get::<AttackComponent>(inflictor_id)
 					.expect(format!("Game logic error: Inflictor can't attack. {}", inflictor_id).as_str()).damage)
		};
		// get name and apply damage to target
		let (target_name, target_health) = {
			// Can unwrap because the target must be alive.
			let (name, health) =
				(self.creatures.get::<NameComponent>(target_id)
						.expect("Game logic error: Victim doesn't have a name").0.clone(),
				self.creatures.get_mut::<HealthComponent>(target_id)
				 		.expect("Game logic error: Victim is immortal."));
			health.damage(damage);
			(name, health.0)
		};
		// english stuff
		let mut direction = AttackDirection::Neutral;

		let inflictor_str = if inflictor_id == PLAYER_ID {
			direction = AttackDirection::FromPlayer;
			"+ You hit".to_owned()
		} else {
			format!("{} hit", name)
		};
		let target_str = if target_id == PLAYER_ID {
			direction = AttackDirection::ToPlayer;
			"you".to_owned()
		} else {
			target_name
		};

		let final_str = format!("{} {} for {} damage.", inflictor_str, target_str, damage.to_string());

		println!("{}", style(final_str)
					   .with(direction.to_color()));

		if target_health > 0 {
			if target_id != PLAYER_ID {
				let final_str = format!("> {} now has {} hitpoints remaining.",
						target_str, target_health.to_string());
				println!("{}", style(final_str).with(Color::Green));
			}
		} else {
			self.die(target_id);
		}
	}
	pub fn die(&mut self, dead_id: CreatureId) {
		let name = self.creatures.remove(dead_id)
				.expect("Game internal error: creature must have existed")
				.remove::<NameComponent>()
				.expect("Creature must have name.").0;

		let target_str = if dead_id == PLAYER_ID {
			style("You died!".to_owned())
					.with(Color::Red)
		} else {
			style(format!("{} has died!", style(name)
					.with(Color::Red)))
					.with(Color::Green)
		};
		println!("{}", target_str);
	}
}

pub fn player_system(state: &mut GameState) {
	// Can unwrap here because the player should exist.
	// If not then why should the game even be running.
	let player_health = state.creatures.get::<HealthComponent>(PLAYER_ID)
			.expect("Game logic error: the player is dead and the game is still running.").0;

	// Player control consists of three phases:
	// 1- Show the enviroment and conditions:
	println!("{}", style(format!("== You have {} hitpoints remaining.", player_health))
			.with(Color::Green));

	let mut creature_string = String::new();

	let mut count = 0usize;

	/* Left for debug purposes later. Might be converted into command.
	for (id, name) in state.creatures.all::<AttackComponent>().iter().enumerate() {
		if let Some(str) = name {
			println!("{}: yes", id);
		} else {
			println!("{}: None", id);
		}
	}*/

	// Can unwrap because alive() ASSURES that the returned creatures are alive.
	for name in state.creatures.alive().iter()
			.filter(|id| **id != PLAYER_ID)
			.map(|id| state.creatures.get::<NameComponent>(*id)
			.expect(format!("Game internal error: creature {} should exist", id).as_str()).0.clone()) {
		creature_string.push_str(
			format!("{}; ", name).as_str()
		);
		count += 1;
	}

	if count == 0 {
		println!("=============== You WIN! ==============");
	} else {
		let stylized = style(format!("== There are {} enemies: {}", count.to_string(), creature_string))
				.with(Color::Red);
		println!("{}", stylized);
	}

	// 2- Ask for player input
	println!("{}", style("Enter a command:")
			.with(Color::DarkGreen));
	loop {
		let chosen = Command::get(state);

		// 3- Process the input.
		match chosen {
			Command::Attack(target) => {
				break state.hit(PLAYER_ID, target);
			}
			Command::Examine(target) => {
				/* Removed until multiple component borrows at the same time in CreatureMap.
				let creature = state.creatures.get(target)
											  .expect("Game logic error: if the player is choosing this creature then it must exist.");
				let stylized = style(format!("{} has {} hitpoints remaining and does {} damage.",
				creature.name, creature.health, creature.damage)).with(Color::Red);
				println!("{}", stylized);*/
			}
			Command::Status => {
				println!("{}", style(format!("== You have {} hitpoints remaining.", player_health))
						.with(Color::Green));
				let stylized = style(format!("== There are {} enemies: {}", count.to_string(), creature_string))
						.with(Color::Red);
				println!("{}", stylized);
			}
			Command::Help => {
				println!("The available commands are:
attack: Hit enemies. Usage: 'attack enemy_name'
examine: Shows the status of a creature. Usage: 'examine enemy_name'
status: Show your character's status and remaining enemies."
				);
			}
			Command::Debug(DebugCommand::Remove(target)) => {
				let name = state.creatures.remove(target)
						.expect("Game internal error: game must have existed")
						.remove::<NameComponent>()
						.expect("Creature must have a name.").0;

				println!("Creature '{}' with the id {} has been removed from the game.", name, target);
			}
		}
		println!("{}", style("Enter another command:")
					   .with(Color::DarkGreen));
	}
}
