use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Teacher {
    id: u64,
    index: u64,
    name: String,
}