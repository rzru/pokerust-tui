use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NamedApiResource {
    pub name: Option<String>,
    pub url: Option<String>,
}

impl NamedApiResource {
    pub fn get_name_or_stub(&self) -> String {
        self.name.as_ref().unwrap_or(&"".to_string()).to_string()
    }
}
