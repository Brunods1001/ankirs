use crate::queries::{create_card, delete_card, list_cards, update_card};
use async_trait::async_trait;
use sqlx::{Sqlite, Transaction};
use std::fmt::Display;
use std::io::{self, Write};
use std::process::Command;

use strum::{Display, EnumIter, IntoEnumIterator};

fn parse_input<T: IntoEnumIterator + Sized>(input: &str) -> Option<T> {
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

trait MenuOptions: IntoEnumIterator + Display + Sized + PartialEq + Clone + Copy {
    fn print_menu() {
        println!("What would you like to do?");
        for option in Self::iter().enumerate() {
            println!("{}. {}", option.0 + 1, option.1);
        }
    }

    fn print(&self) -> Self {
        Self::print_menu();
        *self
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
trait DecisionMaker {
    async fn make_decision(
        self,
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<(MenuState, bool), sqlx::Error>;
    // async fn make_decision<'a>(&'a self, tx: &mut Transaction<'_, Sqlite>) -> Result<(&'a Self, bool), sqlx::Error>;
}

#[derive(EnumIter, Display, Debug, PartialEq, Clone, Copy)]
enum MenuState {
    MainMenu,
    CardMenu,
}

impl MenuOptions for MenuState {}

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
            MenuState::CardMenu => {
                CardMenuOptions::print_menu();
                let card_menu_choice = CardMenuOptions::from_input().unwrap();
                card_menu_choice.make_decision(tx).await
            }
        }
    }
}

#[derive(EnumIter, Display, Debug, PartialEq, Clone, Copy)]
enum CardMenuOptions {
    Create,
    List,
    Update,
    Delete,
    GoToMainMenu,
    Quit,
}

impl MenuOptions for CardMenuOptions {}

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
                create_card(tx, front, back).await?;
            }
            CardMenuOptions::List => {
                println!("Listing all cards");
                list_cards(tx).await?;
            }
            CardMenuOptions::Update => {
                println!("Updating a card");
                let id = prompt_for_card_id()?;
                let (front, back) = prompt_for_card_details()?;
                update_card(tx, id, Some(front), Some(back)).await?;
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
            CardMenuOptions::Quit => {
                println!("Quitting");
                return Ok((MenuState::MainMenu, false));
            }
        };
        Ok((MenuState::CardMenu, true))
    }
}

#[derive(EnumIter, Display, Debug, PartialEq, Clone, Copy)]
enum MenuOption {
    // CardMenu(CardMenuOptions),
    CardMenu,
    Quit,
}

impl MenuOptions for MenuOption {}

#[async_trait]
impl DecisionMaker for MenuOption {
    async fn make_decision(
        self,
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<(MenuState, bool), sqlx::Error> {
        match self {
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

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    } else {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        io::stdout().flush().unwrap();
    }
}

fn prompt_for_card_details() -> Result<(String, String), io::Error> {
    let mut front = String::new();
    let mut back = String::new();

    println!("Front: ");
    // io::stdout().flush()?;
    io::stdin().read_line(&mut front)?;

    println!("Back: ");
    // io::stdout().flush()?;
    io::stdin().read_line(&mut back)?;

    Ok((front.trim().to_string(), back.trim().to_string()))
}

fn prompt_for_card_id() -> Result<i64, io::Error> {
    let mut id = String::new();
    println!("ID: ");
    io::stdin().read_line(&mut id)?;
    Ok(id.trim().parse().unwrap())
}

pub async fn start_app(tx: &mut Transaction<'_, Sqlite>) -> Result<(), sqlx::Error> {
    clear_screen();

    println!("Starting app");

    let mut menu_state = MenuState::MainMenu;

    loop {
        let (next_state, should_continue) = menu_state.make_decision(tx).await?;
        menu_state = next_state;
        if !should_continue {
            break;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_parse_input() {
        use super::parse_input;
        use super::CardMenuOptions;
        assert_eq!(
            parse_input::<CardMenuOptions>("1"),
            Some(CardMenuOptions::Create)
        );
        assert_eq!(
            parse_input::<CardMenuOptions>("2"),
            Some(CardMenuOptions::List)
        );
        assert_eq!(
            parse_input::<CardMenuOptions>("3"),
            Some(CardMenuOptions::Update)
        );
        assert_eq!(
            parse_input::<CardMenuOptions>("4"),
            Some(CardMenuOptions::Delete)
        );
        assert_eq!(
            parse_input::<CardMenuOptions>("5"),
            Some(CardMenuOptions::Quit)
        );
        assert_eq!(parse_input::<CardMenuOptions>("6"), None);
    }
}
