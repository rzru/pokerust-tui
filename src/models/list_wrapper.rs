use serde::{Deserialize, Serialize};

use super::NamedApiResource;

#[derive(Serialize, Deserialize, Debug)]
pub struct ListWrapper {
    pub count: Option<i32>,
    pub results: Option<Vec<NamedApiResource>>,
}
