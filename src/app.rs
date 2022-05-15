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
    Abilities,
}

impl CurrentMainPageState {
    pub fn get_next(&self) -> Self {
        match self {
            Self::VersionGroupSelection => Self::BasicInfo,
            Self::BasicInfo => Self::Abilities,
            Self::Abilities => Self::BasicInfo,
        }
    }
}

pub struct App {
    pub pokemon_list: TestStatefulList,
    http: Http,
    pub current_pokemon: Option<ExtendedPokemonInfo>,
    pub search: String,
    pub selected_part: SelectedPart,
    pub loading: bool,
    pub current_main_page_state: CurrentMainPageState,
    pub pokemon_moves_list_state: SwitchableTableState,
    pub selected_version: Option<NamedApiResource>,
    pub version_groups: TestStatefulList,
}

impl App {
    pub fn new() -> Self {
        Self {
            pokemon_list: StatefulList::with_items(vec![]),
            http: Http::new(),
            current_pokemon: None,
            search: String::new(),
            selected_part: SelectedPart::List,
            loading: false,
            current_main_page_state: CurrentMainPageState::VersionGroupSelection,
            pokemon_moves_list_state: SwitchableTableState::new(),
            selected_version: None,
            version_groups: StatefulList::with_items(vec![]),
        }
    }

    pub async fn fetch_pokemon_list(&self) -> Vec<NamedApiResource> {
        let uri = format!(
            "{}{}{}",
            POKEAPI_DEFAULT_URL, "pokemon", DEFAULT_LIST_QUERY_PARAMS
        );
        let pokemon_list: Option<ListWrapper> = self.http.get_as_object(&uri).await;

        pokemon_list.unwrap().results.unwrap()
    }

    pub async fn fetch_version_groups(&self) -> Vec<NamedApiResource> {
        let uri = format!(
            "{}{}{}",
            POKEAPI_DEFAULT_URL, "version-group", DEFAULT_LIST_QUERY_PARAMS
        );
        let version_groups_list: Option<ListWrapper> = self.http.get_as_object(&uri).await;

        version_groups_list.unwrap().results.unwrap()
    }

    pub async fn fetch_abilities_and_moves(&mut self) {
        if let Some(current_pokemon) = self.current_pokemon.as_mut() {
            let fetch_url =
                |api_resource: &NamedApiResource| api_resource.url.as_ref().unwrap().to_string();

            let (abilities, moves) = (
                current_pokemon.pokemon.abilities.as_ref().unwrap(),
                current_pokemon.pokemon.moves.as_ref().unwrap(),
            );

            let (abilities, moves): (Vec<PokemonAbilityExt>, Vec<PokemonMoveExt>) = join!(
                fetch_external(abilities.as_slice(), |ability| {
                    fetch_url(ability.ability.as_ref().unwrap())
                }),
                fetch_external(moves.as_slice(), |mv| {
                    fetch_url(mv.de_move.as_ref().unwrap())
                }),
            );

            current_pokemon.abilities = abilities;
            current_pokemon.moves = moves;
        }
    }

    pub async fn fetch_pokemon_with_info(&mut self, pokemon: &NamedApiResource) {
        let uri = pokemon.url.as_ref().unwrap().to_string();
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
        self.selected_version = None;
    }

    pub fn filter_list(&mut self) {
        self.pokemon_list.items_to_render = self
            .pokemon_list
            .items
            .iter()
            .filter_map(|pokemon| {
                let should_be_included = pokemon
                    .name
                    .as_ref()
                    .unwrap()
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
}
