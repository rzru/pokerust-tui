use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NamedApiResource {
    pub name: Option<String>,
    pub url: Option<String>,
}
