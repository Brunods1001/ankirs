use crate::app::state::AppState;
use crate::queries::{create_card, create_deck, delete_card, list_cards, update_card, list_decks};
use async_trait::async_trait;
use sqlx::{Sqlite, Transaction};
use std::fmt::Display;
use std::io::{self, Write};

use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(EnumIter, Display, Debug, PartialEq, Clone, Copy)]
pub enum MenuState {
    MainMenu,
    DeckMenu,
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
pub enum DeckMenuOptions {
    Create,
    List,
    Update,
    Delete,
    GoBack(AppState),
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
impl MenuOptions for DeckMenuOptions {}
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
                menu_choice.make_decision(tx).await
            }
            MenuState::DeckMenu => {
                DeckMenuOptions::print_menu();
                let deck_menu_choice = DeckMenuOptions::from_input().unwrap();
                deck_menu_choice.make_decision(tx).await
            }
            MenuState::CardMenu => {
                CardMenuOptions::print_menu();
                let card_menu_choice = CardMenuOptions::from_input().unwrap();
                card_menu_choice.make_decision(tx).await
            }
            MenuState::CardSubMenu => {
                CardSubMenuOptions::print_menu();
                let card_sub_menu_choice = CardSubMenuOptions::from_input().unwrap();
                card_sub_menu_choice.make_decision(tx).await
            }
        }
    }
}

#[async_trait]
impl DecisionMaker for DeckMenuOptions {
    async fn make_decision(
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
            _ => {
                println!("Not implemented");
            }
        }
        Ok((MenuState::DeckMenu, true))
    }
}

#[async_trait]
impl DecisionMaker for CardMenuOptions {
    async fn make_decision(
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
impl DecisionMaker for CardSubMenuOptions {
    async fn make_decision(
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
impl DecisionMaker for MenuOption {
    async fn make_decision(
        self,
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<(MenuState, bool), sqlx::Error> {
        match self {
            MenuOption::DeckMenu => {
                DeckMenuOptions::print_menu();
                let deck_menu_choice = DeckMenuOptions::from_input().unwrap();
                return deck_menu_choice.make_decision(tx).await;
            }
            MenuOption::CardMenu => {
                CardMenuOptions::print_menu();
                let card_menu_choice = CardMenuOptions::from_input().unwrap();
                return card_menu_choice.make_decision(tx).await;
            }
            MenuOption::Quit => {
                return Ok((MenuState::MainMenu, false));
            }
        }
    }
}

trait MenuOptions: IntoEnumIterator + Display + Sized + PartialEq + Clone {
    fn print_menu() {
        println!("What would you like to do?");
        for option in Self::iter().enumerate() {
            println!("{}. {}", option.0 + 1, option.1);
        }
    }

    fn print(&self) -> &Self {
        Self::print_menu();
        self
    }

    fn from_input() -> Option<Self> {
        let mut input = String::new();

        print!("Please enter a command: ");
        io::stdout().flush().unwrap(); // Make sure the prompt is displayed immediately
        io::stdin().read_line(&mut input).ok()?; // Read a line of input

        parse_input(&input)
    }
}

#[async_trait]
pub trait DecisionMaker {
    async fn make_decision(
        self,
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<(MenuState, bool), sqlx::Error>;
    // async fn make_decision<'a>(&'a self, tx: &mut Transaction<'_, Sqlite>) -> Result<(&'a Self, bool), sqlx::Error>;
}

pub fn parse_input<T: IntoEnumIterator + Sized>(input: &str) -> Option<T> {
    // Try to parse the input as a usize
    let num: usize = input.trim().parse().ok()?;

    // Enumerate over each variant and its index
    for (index, variant) in T::iter().enumerate() {
        // If the input number matches the index, return the variant
        if num == index + 1 {
            return Some(variant);
        }
    }

    // If no match was found, return None
    None
}
fn prompt_for_deck_details() -> Result<(String, Option<String>), io::Error> {
    let mut name = String::new();
    let mut description = String::new();

    println!("Name: ");
    io::stdin().read_line(&mut name)?;

    println!("Description: ");
    // io::stdout().flush()?;
    io::stdin().read_line(&mut description)?;

    Ok((name.trim().to_string(), Some(description.trim().to_string())))
}
fn prompt_for_card_details() -> Result<(Option<String>, Option<String>), io::Error> {
    let mut front = String::new();
    let mut back = String::new();

    println!("Front: ");
    // io::stdout().flush()?;
    io::stdin().read_line(&mut front)?;

    println!("Back: ");
    // io::stdout().flush()?;
    io::stdin().read_line(&mut back)?;

    match (front.trim(), back.trim()) {
        ("", "") => Ok((None, None)),
        (front, "") => Ok((Some(front.to_string()), None)),
        ("", back) => Ok((None, Some(back.to_string()))),
        (front, back) => Ok((Some(front.to_string()), Some(back.to_string()))),
    }
}

fn prompt_for_card_id() -> Result<i64, io::Error> {
    let mut id = String::new();
    println!("ID: ");
    io::stdin().read_line(&mut id)?;
    Ok(id.trim().parse().unwrap())
}
