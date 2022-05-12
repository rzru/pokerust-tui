use super::NamedApiResource;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionGameIndex {
    pub game_index: Option<i32>,
    pub version: Option<NamedApiResource>,
}
