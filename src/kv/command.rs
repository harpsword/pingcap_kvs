use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Set(Set),

    Remove(Remove),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Set {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Remove {
    pub key: String,
}
