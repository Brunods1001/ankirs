pub mod card;
mod deck;
pub mod traits;
pub mod utils;

use self::traits::{DecisionMaker, MenuOptions, ProcessOption};
use super::menus::card::{CardMenuOptions, CardSubMenuOptions};
use super::menus::deck::{DeckDetailMenuOptions, DeckMenuOptions};
use super::state::AppState;
use async_trait::async_trait;
use sqlx::SqlitePool;

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
    DeckMenu,
    CardMenu,
    Quit,
}

impl MenuOptions for MenuState {}

#[async_trait]
impl DecisionMaker for MenuState {
    async fn make_decision(
        self,
        pool: &SqlitePool,
        state: &AppState,
    ) -> Result<(MenuState, bool), sqlx::Error> {
        println!("Making decision for {:?}", self);
        match self {
            MenuState::MainMenu => {
                MenuOption::print_menu();
                let menu_choice = MenuOption::from_input().unwrap();
                menu_choice.process(&pool, state).await
            }
            MenuState::DeckMenu => {
                DeckMenuOptions::print_menu();
                let deck_menu_choice = DeckMenuOptions::from_input().unwrap();
                println!("Deck menu choice is {:?}", deck_menu_choice);
                deck_menu_choice.process(&pool, state).await
            }
            MenuState::DeckDetailMenu(id) => {
                println!("Inside menu state: DeckDetailMenu ID: {}", id);
                DeckDetailMenuOptions::print_menu();
                let deck_detail_menu_choice = DeckDetailMenuOptions::from_input().unwrap();
                let deck_detail_menu_choice = match deck_detail_menu_choice {
                    DeckDetailMenuOptions::View(_) => DeckDetailMenuOptions::View(id),
                    DeckDetailMenuOptions::ListCards(_) => DeckDetailMenuOptions::ListCards(id),
                    DeckDetailMenuOptions::AddCard(_) => DeckDetailMenuOptions::AddCard(id),
                    DeckDetailMenuOptions::ListAllCards(_) => {
                        DeckDetailMenuOptions::ListAllCards(id)
                    }
                    DeckDetailMenuOptions::Review(_) => DeckDetailMenuOptions::Review(id),
                    DeckDetailMenuOptions::CreateCard(_) => DeckDetailMenuOptions::CreateCard(id),
                    DeckDetailMenuOptions::GoBack(_) => {
                        DeckDetailMenuOptions::GoBack(state.clone())
                    }
                    DeckDetailMenuOptions::Quit => DeckDetailMenuOptions::Quit,
                };

                println!("CHOOSING DECK DETAIL MENU {:?}", deck_detail_menu_choice);

                deck_detail_menu_choice.process(&pool, state).await
            }
            MenuState::CardMenu => {
                CardMenuOptions::print_menu();
                let card_menu_choice = CardMenuOptions::from_input().unwrap();
                card_menu_choice.process(&pool, state).await
            }
            MenuState::CardSubMenu => {
                CardSubMenuOptions::print_menu();
                let card_sub_menu_choice = CardSubMenuOptions::from_input().unwrap();
                card_sub_menu_choice.process(&pool, state).await
            }
        }
    }
}

impl MenuOptions for MenuOption {}

#[async_trait]
impl ProcessOption for MenuOption {
    async fn process(
        self,
        pool: &SqlitePool,
        state: &AppState,
    ) -> Result<(MenuState, bool), sqlx::Error> {
        match self {
            MenuOption::DeckMenu => {
                DeckMenuOptions::print_menu();
                let deck_menu_choice = DeckMenuOptions::from_input().unwrap();
                return deck_menu_choice.process(&pool, state).await;
            }
            MenuOption::CardMenu => {
                CardMenuOptions::print_menu();
                let card_menu_choice = CardMenuOptions::from_input().unwrap();
                return card_menu_choice.process(&pool, state).await;
            }
            MenuOption::Quit => {
                return Ok((MenuState::MainMenu, false));
            }
        }
    }
}
