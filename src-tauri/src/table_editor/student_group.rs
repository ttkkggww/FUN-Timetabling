use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StudentGroup {
    pub id: String,
    pub name: String,
}
