use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Annotation {
    pub kind: String,
    pub content: String,
}
