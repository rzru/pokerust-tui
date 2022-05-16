use tokio::join;

use crate::{
    http::{fetch_external, Http},
    models::{ExtendedPokemonInfo, NamedApiResource, Pokemon, PokemonMoveExt, PokemonSpecies},
    models::{ListWrapper, PokemonAbilityExt},
    stateful_list::StatefulList,
    switchable_table_state::SwitchableTableState,
    DEFAULT_LIST_QUERY_PARAMS, POKEAPI_DEFAULT_URL,
};

pub type TestStatefulList = StatefulList<NamedApiResource>;

pub enum SelectedPart {
    List,
    Main,
}

pub enum CurrentMainPageState {
    BasicInfo,
    VersionGroupSelection,
}

pub struct App {
    http: Http,
    pub search: String,
    pub loading: bool,
    pub pokemon_list: TestStatefulList,
    pub selected_part: SelectedPart,
    pub version_groups: TestStatefulList,
    pub current_pokemon: Option<ExtendedPokemonInfo>,
    pub selected_version_group: Option<NamedApiResource>,
    pub rendered_moves_count: Option<usize>,
    pub current_main_page_state: CurrentMainPageState,
    pub pokemon_moves_list_state: SwitchableTableState,
}

impl App {
    pub fn new() -> Self {
        Self {
            http: Http::new(),
            search: String::new(),
            loading: false,
            pokemon_list: StatefulList::with_items(vec![]),
            selected_part: SelectedPart::List,
            version_groups: StatefulList::with_items(vec![]),
            current_pokemon: None,
            selected_version_group: None,
            rendered_moves_count: None,
            current_main_page_state: CurrentMainPageState::VersionGroupSelection,
            pokemon_moves_list_state: SwitchableTableState::new(),
        }
    }

    pub async fn init(&mut self) {
        let (pokemon_list, version_groups) =
            join!(self.fetch_list("pokemon"), self.fetch_list("version-group"));
        self.set_pokemon_list_and_version_groups(pokemon_list, version_groups);
    }

    pub async fn fetch_list(&self, list_name: &str) -> Vec<NamedApiResource> {
        let uri = format!(
            "{}{}{}",
            POKEAPI_DEFAULT_URL, list_name, DEFAULT_LIST_QUERY_PARAMS
        );
        let results: Option<ListWrapper> = self.http.get_as_object(&uri).await;

        results
            .and_then(|list_wrapper| list_wrapper.results)
            .unwrap_or(vec![])
    }

    pub async fn fetch_abilities_and_moves(&mut self) {
        if let Some(current_pokemon) = self.current_pokemon.as_mut() {
            let empty_abilities = vec![];
            let empty_moves = vec![];
            let fetch_url = |api_resource: &NamedApiResource| {
                api_resource
                    .url
                    .as_ref()
                    .unwrap_or(&"".to_string())
                    .to_string()
            };

            let (abilities, moves) = (
                current_pokemon
                    .pokemon
                    .abilities
                    .as_ref()
                    .unwrap_or(&empty_abilities),
                current_pokemon
                    .pokemon
                    .moves
                    .as_ref()
                    .unwrap_or(&empty_moves),
            );

            let (abilities, moves): (Vec<PokemonAbilityExt>, Vec<PokemonMoveExt>) = join!(
                fetch_external(abilities.as_slice(), |ability| {
                    // TODO: replace unwrap with something better
                    fetch_url(ability.ability.as_ref().unwrap())
                }),
                fetch_external(moves.as_slice(), |mv| {
                    // TODO: replace unwrap with something better
                    fetch_url(mv.de_move.as_ref().unwrap())
                }),
            );

            current_pokemon.abilities = abilities;
            current_pokemon.moves = moves;
        }
    }

    pub async fn fetch_pokemon_with_info(&mut self, pokemon: &NamedApiResource) {
        let uri = pokemon.url.as_ref().unwrap_or(&"".to_string()).to_string();
        if uri.is_empty() {
            return;
        }

        let pokemon: Option<Pokemon> = self.http.get_as_object(&uri).await;

        if let Some(pokemon) = pokemon {
            let mut species: Option<PokemonSpecies> = None;
            let species_url = pokemon
                .species
                .as_ref()
                .and_then(|species| species.url.as_ref());

            if let Some(species_url) = species_url {
                species = self.http.get_as_object(&species_url).await;
            }

            self.current_pokemon = Some(ExtendedPokemonInfo {
                pokemon,
                abilities: vec![],
                moves: vec![],
                species: species.unwrap(),
            });
        }
    }

    pub fn set_pokemon_list_and_version_groups(
        &mut self,
        pokemon_list: Vec<NamedApiResource>,
        version_groups: Vec<NamedApiResource>,
    ) {
        self.pokemon_list.items.extend(pokemon_list.clone());
        self.pokemon_list.items_to_render.extend(pokemon_list);
        self.version_groups.items_to_render.extend(version_groups);
    }

    pub fn reset_current_pokemon(&mut self) {
        self.current_pokemon = None;
        self.current_main_page_state = CurrentMainPageState::VersionGroupSelection;
        self.selected_part = SelectedPart::List;
        self.selected_version_group = None;
    }

    pub fn filter_list(&mut self) {
        self.pokemon_list.items_to_render = self
            .pokemon_list
            .items
            .iter()
            .filter_map(|pokemon| {
                let should_be_included = pokemon
                    .get_name_or_stub()
                    .to_lowercase()
                    .contains(&self.search.to_lowercase());

                if should_be_included {
                    return Some(pokemon.clone());
                }

                None
            })
            .collect();

        self.pokemon_list.state.select(Some(0));
    }

    pub async fn on_pokemon_selected<F>(&mut self, redraw: F)
    where
        F: FnOnce(&mut Self),
    {
        let pokemon = self.pokemon_list.get_selected().cloned();
        if let Some(pokemon) = pokemon {
            self.reset_current_pokemon();
            self.loading = true;
            redraw(self);
            self.fetch_pokemon_with_info(&pokemon).await;
            self.loading = false;
            self.selected_part = SelectedPart::Main;
        }
    }

    pub fn on_version_group_selected(&mut self) {
        self.selected_version_group = self
            .version_groups
            .get_selected()
            .and_then(|t| Some(t.clone()));

        self.current_main_page_state = CurrentMainPageState::BasicInfo;
    }

    pub async fn on_moves_and_abilities_open<F>(&mut self, redraw: F)
    where
        F: FnOnce(&mut Self),
    {
        self.pokemon_moves_list_state = SwitchableTableState::new();
        if let Some(current_pokemon) = self.current_pokemon.as_ref() {
            if current_pokemon.abilities.is_empty() && current_pokemon.moves.is_empty() {
                self.loading = true;
                self.rendered_moves_count = None;
                redraw(self);
                self.fetch_abilities_and_moves().await;
                self.loading = false;
            }
        }
    }

    pub fn on_search_append(&mut self, character: char) {
        self.search.push(character);
        self.filter_list();
    }

    pub fn on_search_remove(&mut self) {
        self.search.pop();
        self.filter_list();
    }
}
