mod deck;
pub mod traits;
pub mod utils;

use self::traits::{DecisionMaker, MenuOptions, ProcessOption};
use super::menus::deck::{DeckDetailMenuOptions, DeckMenuOptions};
use super::state::AppState;
use super::menus::utils::{prompt_for_card_id, prompt_for_card_details};

use crate::queries::{create_card, delete_card, list_cards, update_card};
use async_trait::async_trait;
use sqlx::{Sqlite, Transaction};

use strum::{Display, EnumIter};

#[derive(EnumIter, Display, Debug, PartialEq, Clone, Copy)]
pub enum MenuState {
    MainMenu,
    DeckMenu,
    DeckDetailMenu(i64),
    CardMenu,
    CardSubMenu,
}

#[derive(EnumIter, Display, Debug, PartialEq, Clone, Copy)]
enum MenuOption {
    // CardMenu(CardMenuOptions),
    DeckMenu,
    CardMenu,
    Quit,
}

#[derive(EnumIter, Display, Debug, PartialEq, Clone)]
pub enum CardMenuOptions {
    Create,
    List,
    Update,
    Delete,
    GoToMainMenu,
    GoToSubMenu,
    GoBack(AppState),
    Quit,
}

#[derive(EnumIter, Display, Debug, PartialEq, Clone, Copy)]
enum CardSubMenuOptions {
    Front,
    Back,
    GoToCardMenu,
    Quit,
}

impl MenuOptions for MenuState {}
impl MenuOptions for CardMenuOptions {}
impl MenuOptions for CardSubMenuOptions {}

#[async_trait]
impl DecisionMaker for MenuState {
    async fn make_decision(
        self,
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<(MenuState, bool), sqlx::Error> {
        println!("Making decision for {:?}", self);
        match self {
            MenuState::MainMenu => {
                MenuOption::print_menu();
                let menu_choice = MenuOption::from_input().unwrap();
                menu_choice.process(tx).await
            }
            MenuState::DeckMenu => {
                DeckMenuOptions::print_menu();
                let deck_menu_choice = DeckMenuOptions::from_input().unwrap();
                println!("Deck menu choice is {:?}", deck_menu_choice);
                deck_menu_choice.process(tx).await
            }
            MenuState::DeckDetailMenu(id) => {
                println!("Inside menu state: DeckDetailMenu ID: {}", id);
                DeckDetailMenuOptions::print_menu();
                let deck_detail_menu_choice = DeckDetailMenuOptions::from_input().unwrap();
                let deck_detail_menu_choice = match deck_detail_menu_choice {
                    DeckDetailMenuOptions::View(_) => DeckDetailMenuOptions::View(id),
                    DeckDetailMenuOptions::GoBack(state) => DeckDetailMenuOptions::GoBack(state),
                    DeckDetailMenuOptions::Quit => DeckDetailMenuOptions::Quit,
                };

                println!("CHOOSING DECK DETAIL MENU {:?}", deck_detail_menu_choice);

                deck_detail_menu_choice.process(tx).await
            }
            MenuState::CardMenu => {
                CardMenuOptions::print_menu();
                let card_menu_choice = CardMenuOptions::from_input().unwrap();
                card_menu_choice.process(tx).await
            }
            MenuState::CardSubMenu => {
                CardSubMenuOptions::print_menu();
                let card_sub_menu_choice = CardSubMenuOptions::from_input().unwrap();
                card_sub_menu_choice.process(tx).await
            }
        }
    }
}

#[async_trait]
impl ProcessOption for CardMenuOptions {
    async fn process(
        self,
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<(MenuState, bool), sqlx::Error> {
        match self {
            CardMenuOptions::Create => {
                println!("Creating a card");
                let (front, back) = prompt_for_card_details()?;

                create_card(
                    tx,
                    front.unwrap_or("".to_string()),
                    back.unwrap_or("".to_string()),
                )
                .await?;
            }
            CardMenuOptions::List => {
                println!("Listing all cards");
                list_cards(tx).await?;
            }
            CardMenuOptions::Update => {
                println!("Updating a card");
                let id = prompt_for_card_id()?;
                let (front, back) = prompt_for_card_details()?;
                update_card(tx, id, front, back).await?;
            }
            CardMenuOptions::Delete => {
                println!("Deleting a card");
                let id = prompt_for_card_id()?;
                delete_card(tx, id).await?;
            }
            CardMenuOptions::GoToMainMenu => {
                println!("Going to main menu");
                return Ok((MenuState::MainMenu, true));
            }
            CardMenuOptions::GoToSubMenu => {
                println!("Going to sub menu");
                return Ok((MenuState::CardSubMenu, true));
            }
            CardMenuOptions::Quit => {
                println!("Quitting");
                return Ok((MenuState::MainMenu, false));
            }
            CardMenuOptions::GoBack(mut state) => {
                println!("Going back");
                // state.go_back();
                let previous_menu = state.get_previous_menu();
                return Ok((previous_menu, true));
            }
        };
        Ok((MenuState::CardMenu, true))
    }
}

#[async_trait]
impl ProcessOption for CardSubMenuOptions {
    async fn process(
        self,
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<(MenuState, bool), sqlx::Error> {
        match self {
            CardSubMenuOptions::Front => {
                println!("Updating the front of a card");
                let id = prompt_for_card_id()?;
                let front = "test".to_string();
                update_card(tx, id, Some(front), None).await?;
            }
            CardSubMenuOptions::Back => {
                println!("Updating the back of a card");
                let id = prompt_for_card_id()?;
                let back = "test".to_string();
                update_card(tx, id, None, Some(back)).await?;
            }
            CardSubMenuOptions::GoToCardMenu => {
                println!("Going to card menu");
                return Ok((MenuState::CardMenu, true));
            }
            CardSubMenuOptions::Quit => {
                println!("Quitting");
                return Ok((MenuState::MainMenu, false));
            }
        };
        Ok((MenuState::CardSubMenu, true))
    }
}

impl MenuOptions for MenuOption {}

#[async_trait]
impl ProcessOption for MenuOption {
    async fn process(
        self,
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<(MenuState, bool), sqlx::Error> {
        match self {
            MenuOption::DeckMenu => {
                DeckMenuOptions::print_menu();
                let deck_menu_choice = DeckMenuOptions::from_input().unwrap();
                return deck_menu_choice.process(tx).await;
            }
            MenuOption::CardMenu => {
                CardMenuOptions::print_menu();
                let card_menu_choice = CardMenuOptions::from_input().unwrap();
                return card_menu_choice.process(tx).await;
            }
            MenuOption::Quit => {
                return Ok((MenuState::MainMenu, false));
            }
        }
    }
}
