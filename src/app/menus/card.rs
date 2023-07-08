use super::traits::{MenuOptions, ProcessOption};
use super::MenuState;
use super::utils::{prompt_for_card_details, prompt_for_card_id};

use crate::app::state::AppState;
use crate::queries::{create_card, delete_card, list_cards, update_card};
use async_trait::async_trait;
use sqlx::{Sqlite, Transaction};

use strum::{Display, EnumIter};

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
pub enum CardSubMenuOptions {
    Front,
    Back,
    GoToCardMenu,
    Quit,
}
impl MenuOptions for CardMenuOptions {}
impl MenuOptions for CardSubMenuOptions {}

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
