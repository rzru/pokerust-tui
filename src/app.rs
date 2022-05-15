use tokio::join;
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::Row,
};

use crate::{
    http::{fetch_external, Http},
    models::{pokemon_move::PokemonMoveExt, NamedApiResource, Pokemon, PokemonSpecies},
    models::{PokemonAbilityExt, PokemonListWrapper},
    stateful_list::StatefulList,
    POKEAPI_DEFAULT_URL,
};

pub type TestStatefulList = StatefulList<NamedApiResource>;

pub enum SelectedPart {
    List,
    Main,
}

pub enum CurrentMainPageState {
    BasicInfo,
    Abilities,
}

impl CurrentMainPageState {
    pub fn get_next(&self) -> Self {
        match self {
            Self::BasicInfo => Self::Abilities,
            Self::Abilities => Self::BasicInfo,
        }
    }
}

pub struct ExtendedPokemonInfo {
    pub pokemon: Pokemon,
    pub abilities: Vec<PokemonAbilityExt>,
    pub moves: Vec<PokemonMoveExt>,
    pub species: PokemonSpecies,
}

impl ExtendedPokemonInfo {
    pub fn get_renderable_abilities(&self) -> Vec<Row> {
        self.pokemon
            .abilities
            .as_ref()
            .unwrap()
            .iter()
            .filter_map(|ability| {
                let extended_ability = self.abilities.iter().find(|extended_ability| {
                    if let Some(item_ability) = ability.ability.as_ref() {
                        return extended_ability.name.as_ref().unwrap_or(&"".to_string())
                            == item_ability.name.as_ref().unwrap_or(&"".to_string());
                    }

                    false
                });

                ability.get_renderable_as_row(extended_ability)
            })
            .collect()
    }

    pub fn get_renderable_basic_info_items(&self) -> Vec<Row> {
        vec![
            Row::new(vec![
                Span::styled(format!("\u{A0}ID"), Style::default().fg(Color::Blue)),
                self.pokemon.get_renderable_id(),
            ]),
            Row::new(vec![
                Span::styled(format!("\u{A0}Order"), Style::default().fg(Color::Blue)),
                self.pokemon.get_renderable_order(),
            ]),
            Row::new(vec![
                Span::styled(format!("\u{A0}Name"), Style::default().fg(Color::Blue)),
                self.pokemon.get_renderable_name(),
            ]),
            Row::new(vec![
                Spans::from(Span::styled(
                    format!("\u{A0}Types"),
                    Style::default().fg(Color::Blue),
                )),
                Spans::from(self.pokemon.get_renderable_types()),
            ]),
            Row::new(vec![
                Span::styled(format!("\u{A0}Height"), Style::default().fg(Color::Blue)),
                self.pokemon.get_renderable_height(),
            ]),
            Row::new(vec![
                Span::styled(format!("\u{A0}Weight"), Style::default().fg(Color::Blue)),
                self.pokemon.get_renderable_weight(),
            ]),
            Row::new(vec![
                Span::styled(
                    format!("\u{A0}Base Experience"),
                    Style::default().fg(Color::Blue),
                ),
                self.pokemon.get_renderable_base_experience(),
            ]),
            Row::new(vec![
                Span::styled(
                    format!("\u{A0}Base Happiness"),
                    Style::default().fg(Color::Blue),
                ),
                self.species.get_renderable_base_happiness(),
            ]),
            Row::new(vec![
                Span::styled(
                    format!("\u{A0}Capture Rate"),
                    Style::default().fg(Color::Blue),
                ),
                self.species.get_renderable_capture_rate(),
            ]),
            Row::new(vec![
                Span::styled(format!("\u{A0}Color"), Style::default().fg(Color::Blue)),
                self.species.get_renderable_color(),
            ]),
            Row::new(vec![
                Span::styled(
                    format!("\u{A0}Is Legendary"),
                    Style::default().fg(Color::Blue),
                ),
                self.species.get_renderable_is_legendary(),
            ]),
        ]
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
            current_main_page_state: CurrentMainPageState::BasicInfo,
        }
    }

    pub async fn fetch_pokemon_list(&mut self) {
        let uri = format!("{}{}", POKEAPI_DEFAULT_URL, "pokemon?limit=100000&offset=0");
        let pokemon_list: Option<PokemonListWrapper> = self.http.get_as_object(&uri).await;
        let pokemon_list = pokemon_list.unwrap().results.unwrap();

        self.pokemon_list.items.extend(pokemon_list.clone());
        self.pokemon_list.items_to_render.extend(pokemon_list);
    }

    pub async fn fetch_pokemon_with_info(&mut self, pokemon: &NamedApiResource) {
        let uri = pokemon.url.as_ref().unwrap().to_string();
        let pokemon: Option<Pokemon> = self.http.get_as_object(&uri).await;

        if let Some(pokemon) = pokemon {
            let fetch_url =
                |api_resource: &NamedApiResource| api_resource.url.as_ref().unwrap().to_string();

            let (abilities, moves) = (
                pokemon.abilities.as_ref().unwrap(),
                pokemon.moves.as_ref().unwrap(),
            );

            let (abilities, moves, species): (
                Vec<PokemonAbilityExt>,
                Vec<PokemonMoveExt>,
                Option<PokemonSpecies>,
            ) = join!(
                fetch_external(abilities.as_slice(), |ability| {
                    fetch_url(ability.ability.as_ref().unwrap())
                }),
                fetch_external(moves.as_slice(), |mv| {
                    fetch_url(mv.de_move.as_ref().unwrap())
                }),
                self.http
                    .get_as_object(pokemon.species.as_ref().unwrap().url.as_ref().unwrap()),
            );

            self.current_pokemon = Some(ExtendedPokemonInfo {
                pokemon,
                abilities,
                moves,
                species: species.unwrap(),
            });
        }
    }

    pub fn reset_current_pokemon(&mut self) {
        self.current_pokemon = None;
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
