use crate::{
    http::Http,
    pokemon::Pokemon,
    pokemon_list::{PokemonFromList, PokemonListWrapper},
    stateful_list::StatefulList,
    POKEAPI_DEFAULT_URL,
};

pub type TestStatefulList = StatefulList<PokemonFromList>;

pub struct App {
    pub pokemon_list: TestStatefulList,
    http: Http,
    pub current_pokemon: Option<Pokemon>,
}

impl App {
    pub fn new() -> Self {
        Self {
            pokemon_list: StatefulList::with_items(vec![]),
            http: Http::new(),
            current_pokemon: None,
        }
    }

    pub async fn fetch_pokemon_list(&mut self) {
        let uri = format!("{}{}", POKEAPI_DEFAULT_URL, "pokemon?limit=100000&offset=0");
        let pokemon_list: Option<PokemonListWrapper> = self.http.get_as_object(&uri).await;

        self.pokemon_list
            .items
            .extend(pokemon_list.unwrap().results.unwrap())
    }

    pub async fn fetch_pokemon_with_info(&mut self, pokemon: &PokemonFromList) {
        let uri = pokemon.url.as_ref().unwrap().to_string();

        self.current_pokemon = self.http.get_as_object(&uri).await;
    }

    pub fn reset_current_pokemon(&mut self) {
        self.current_pokemon = None;
    }
}
