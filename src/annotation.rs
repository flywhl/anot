use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Annotation {
    pub kind: String,
    pub content: String,
}
