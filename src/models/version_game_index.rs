use serde::{Deserialize, Serialize};

use super::NamedApiResource;

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionGameIndex {
    pub game_index: Option<i32>,
    pub version: Option<NamedApiResource>,
}
