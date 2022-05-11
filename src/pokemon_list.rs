use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pokemon {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonListWrapper {
    pub count: Option<i32>,
    pub results: Option<Vec<Pokemon>>,
}
