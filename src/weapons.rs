#[derive(Clone)]
pub struct Weapon {
    pub name: String,
    pub demage: u32
    
}

pub struct WeaponManager {
    count : u32,
    availible_weapons : Vec<Weapon>
}

impl WeaponManager {
    pub fn new() -> WeaponManager {
        WeaponManager {
            count : 0,
            availible_weapons : Vec::new()
        }
    }

    pub fn add_weapon(&mut self, mut new_weapon : Weapon){

    }
}