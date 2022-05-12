use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct APIResource {
    pub url: String,
}
