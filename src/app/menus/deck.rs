use super::traits::{MenuOptions, ProcessOption};
use super::utils::{parse_input, prompt_for_deck_details, prompt_for_deck_id, prompt_for_card_id};
use super::MenuState;
use crate::app::state::AppState;
use crate::queries::{
    create_deck, delete_deck, list_decks, query_deck_exists, query_deck_info, update_deck, list_cards_for_deck, add_card_to_deck, list_cards,
};
use async_trait::async_trait;
use sqlx::{Sqlite, Transaction};
use std::io::{self, Write};

use strum::{Display, EnumIter};

#[derive(EnumIter, Display, Debug, PartialEq, Clone)]
pub enum DeckMenuOptions {
    Create,
    List,
    Update,
    Delete,
    ChooseDeck,
}

#[derive(EnumIter, Display, Debug, PartialEq, Clone)]
pub enum DeckDetailMenuOptions {
    View(i64),
    ListAllCards(i64),
    ListCards(i64),
    AddCard(i64),
    GoBack(AppState),
    Quit,
}

impl MenuOptions for DeckMenuOptions {}
impl MenuOptions for DeckDetailMenuOptions {
    fn from_input() -> Option<Self> {
        let mut input = String::new();

        print!("Please enter a command: ");
        io::stdout().flush().unwrap(); // Make sure the prompt is displayed immediately
        io::stdin().read_line(&mut input).ok()?; // Read a line of input

        parse_input(&input)
    }
}
#[async_trait]
impl ProcessOption for DeckMenuOptions {
    async fn process(
        self,
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<(MenuState, bool), sqlx::Error> {
        match self {
            DeckMenuOptions::Create => {
                println!("Creating a deck");
                let (name, description) = prompt_for_deck_details()?;
                create_deck(tx, name, description).await?;
            }
            DeckMenuOptions::Update => {
                println!("Updating a deck... insert an id");
                let id = prompt_for_deck_id()?;
                println!("Updating deck with id {}", id);
                let (name, description) = prompt_for_deck_details()?;
                update_deck(tx, id, name, description).await?;
            }
            DeckMenuOptions::Delete => {
                println!("Deleting a deck... insert an id");
                let id = prompt_for_deck_id()?;
                println!("Deleting deck with id {}", id);
                delete_deck(tx, id).await?;
            }
            DeckMenuOptions::List => {
                println!("Listing all decks");
                list_decks(tx).await?;
            }
            DeckMenuOptions::ChooseDeck => {
                println!("Choosing a deck");
                let id = prompt_for_deck_id()?;
                println!("Chose deck with id {}", id);
                // check if deck exists first
                let does_exist: bool = query_deck_exists(tx, id).await?;
                if !does_exist {
                    println!("Deck does not exist");
                } else {
                    println!("Deck exists");
                    return Ok((MenuState::DeckDetailMenu(id), true));
                }

                return Ok((MenuState::DeckMenu, true));
            }
        }
        Ok((MenuState::DeckMenu, true))
    }
}

#[async_trait]
impl ProcessOption for DeckDetailMenuOptions {
    async fn process(
        self,
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<(MenuState, bool), sqlx::Error> {
        println!("Making DeckDetailMenuOptions decision for {:?}", self);
        match self {
            DeckDetailMenuOptions::View(id) => {
                println!("Viewing a deck with id {}", id);
                let deck_info: String = query_deck_info(tx, id).await;
                println!("Deck info: {deck_info}");
                return Ok((MenuState::DeckDetailMenu(id), true));
            }
            DeckDetailMenuOptions::ListAllCards(id) => {
                println!("Listing all cards");
                list_cards(tx).await?;
                return Ok((MenuState::DeckDetailMenu(id), true));
            }
            DeckDetailMenuOptions::ListCards(id) => {
                println!("Listing cards for deck with id {}", id);
                list_cards_for_deck(tx, id).await?;
                return Ok((MenuState::DeckDetailMenu(id), true));
            }
            DeckDetailMenuOptions::AddCard(id) => {
                println!("Adding a card to deck with id {}", id);
                let card_id = prompt_for_card_id()?;
                add_card_to_deck(tx, card_id, id).await?;
                return Ok((MenuState::DeckDetailMenu(id), true));
            }
            DeckDetailMenuOptions::GoBack(mut state) => {
                println!("Quitting DeckDetailMenuOptions");
                let previous_menu = state.get_previous_menu();
                println!("MY STATE {:?}", state);
                println!("Going to the previous menu: {previous_menu}");
                return Ok((previous_menu, true));
            }
            DeckDetailMenuOptions::Quit => {
                println!("Quitting DeckDetailMenuOptions");
                return Ok((MenuState::DeckMenu, false));
            }
        }
    }
}
