use crate::game_state::{GameState, PLAYER_ID};
use crate::components::{Entity, Component, ComponentType};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct NeutralComponent {
    target: Option<Entity>
}

impl Component for NeutralComponent {
	fn purpose() -> ComponentType { ComponentType::Creature }
}
impl NeutralComponent {
    pub fn new() -> NeutralComponent {
        NeutralComponent {
            target: None
        }
    }
    // This function should be called when the creature gets hit, so it becomes aggressive towards
    // that creature.
    pub fn deneutralize(&mut self, inflictor: Entity) {
        self.target = Some(inflictor);
    }
}
pub fn neutral(state: &mut GameState) {
    let ids: Vec<usize> = state.creatures.all::<NeutralComponent>()
            .iter().enumerate()
            .filter_map(|(id, n)| {
                if let Some(c) = n {
                    if let Some(t) = c.target {
                        Some(id)
                    } else { None }
                } else { None }
            }).collect();
    for id in ids {
        state.hit(id, PLAYER_ID);
    }
}
