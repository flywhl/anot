use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Annotation {
    pub kind: String,
    pub content: String,
}
