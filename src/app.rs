use crate::{
    http::Http,
    pokemon_list::{Pokemon, PokemonListWrapper},
    stateful_list::StatefulList,
    POKEAPI_DEFAULT_URL,
};

pub type TestStatefulList = StatefulList<Pokemon>;

pub struct App {
    pub stateful_list: TestStatefulList,
    http: Http,
}

impl App {
    pub fn new() -> Self {
        Self {
            stateful_list: StatefulList::with_items(vec![]),
            http: Http::new(),
        }
    }

    pub async fn fetch_list(&mut self) {
        let uri = format!("{}{}", POKEAPI_DEFAULT_URL, "pokemon?limit=100000&offset=0");
        let pokemon_list: Option<PokemonListWrapper> = self.http.get_as_object(&uri).await;

        self.stateful_list
            .items
            .extend(pokemon_list.unwrap().results.unwrap())
    }
}
