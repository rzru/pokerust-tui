use serde::{Deserialize, Serialize};

use super::NamedApiResource;

#[derive(Serialize, Deserialize, Debug)]
pub struct FlavorTextEntry {
    pub flavor_text: Option<String>,
    pub version_group: Option<NamedApiResource>,
    pub language: Option<NamedApiResource>,
}
