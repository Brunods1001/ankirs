use super::MenuState;
use super::traits::{MenuOptions, ProcessOption};
use super::utils::{prompt_for_deck_details, prompt_for_deck_id, parse_input};
use crate::app::state::AppState;
use crate::queries::{
    create_deck, list_decks, query_deck_exists,
    query_deck_info
};
use async_trait::async_trait;
use sqlx::{Sqlite, Transaction};
use std::io::{self, Write};

use strum::{Display, EnumIter};

#[derive(EnumIter, Display, Debug, PartialEq, Clone)]
pub enum DeckMenuOptions {
    Create,
    List,
    // Update,
    // Delete,
    ChooseDeck,
}

#[derive(EnumIter, Display, Debug, PartialEq, Clone)]
pub enum DeckDetailMenuOptions {
    View(i64),
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
