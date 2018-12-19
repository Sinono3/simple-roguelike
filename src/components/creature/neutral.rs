use crate::game_state::GameState;
use crate::components::{Entity, Component, ComponentType};

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
    let ids: Vec<(Entity, Entity)> = state.creatures.all::<NeutralComponent>()
            .iter().enumerate()
            .filter_map(|(id, n)| {
                if let Some(c) = n {
                    if let Some(t) = c.target {
                        Some((id, t))
                    } else { None }
                } else { None }
            }).collect();
    for (id, target) in ids {
        state.hit(id, target);
    }
}
