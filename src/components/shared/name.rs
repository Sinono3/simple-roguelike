use specs::{Component, VecStorage};

#[derive(Component, Debug, Default, Deserialize, Serialize)]
#[storage(VecStorage)]
pub struct Name(String);

impl Name {
    pub fn new(name: &str) -> Self {
        Name(name.to_owned())
    }
    pub fn get(&self) -> &str {
        self.0.as_str()
    }
}
