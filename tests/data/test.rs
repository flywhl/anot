pub struct Example {
    // @todo: Add more fields
    name: String,
}

impl Example {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(), // @fixme: This needs better error handling
        }
    }
}
