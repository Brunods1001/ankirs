use super::utils::parse_input;
use super::MenuState;

use async_trait::async_trait;
use sqlx::{Sqlite, Transaction};
use std::fmt::Display;
use std::io::{self, Write};
use strum::IntoEnumIterator;

pub trait MenuOptions: IntoEnumIterator + Display + Sized + PartialEq + Clone {
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

#[async_trait]
pub trait ProcessOption {
    async fn process(
        self,
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<(MenuState, bool), sqlx::Error>;
}
