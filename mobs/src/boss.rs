#[derive(Debug, PartialEq)]
pub struct Boss {
    pub name: String,
    pub age: u32,
}

impl Boss {
    pub fn new(name: &str, age: u32) -> Self {
        Boss {
            name: name.to_string(),
            age,
        }
    }
}