use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Uuid(String);

impl Uuid {
    pub fn new(uuid: &str) -> Self {
        uuid::Uuid::try_parse(uuid).expect("Invalid uuid");
        Uuid(uuid.to_string())
    }

    pub fn generate() -> Self {
        Uuid(uuid::Uuid::new_v4().to_string())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
