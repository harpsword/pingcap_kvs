use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    // Get {
    //     key: String,
    // },

    Set {
        key: String,
        value: String,
    },

    Remove {
        key: String
    },
}