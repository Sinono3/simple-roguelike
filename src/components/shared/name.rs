use specs::{Component, VecStorage};

#[derive(Debug, Deserialize, Serialize)]
enum NameKind {
    Unique,
    NonUnique,
        //Repetition(bool, i32),
}

#[derive(Component, Debug, Deserialize, Serialize)]
#[storage(VecStorage)]
// First member represents raw name and the bool represents if it has been presented before.
pub struct Name {
    raw_name: String,
    kind: NameKind
}

impl Name {
    pub fn new(name: &str, unique: bool) -> Self {
        Name {
            raw_name: name.to_owned(),
            kind: if unique {
                NameKind::Unique
            } else {
                NameKind::NonUnique
            }
        }
    }
    pub fn raw(&self) -> &str {
        self.raw_name.as_str()
    }
    pub fn get(&self) -> String {
        match self.kind {
            NameKind::Unique => self.raw_name.clone(),
            // 'a' and 'an' articles not covered because they should be put in the spawn event.
            NameKind::NonUnique => format!("the {}", self.raw_name),
        }
    }
}
