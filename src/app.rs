use crate::queries::{create_card, delete_card, list_cards, update_card};
use sqlx::{Sqlite, Transaction};
use std::io::{self, Write};
use std::process::Command;
use std::fmt::Display;

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

trait MenuOptions: IntoEnumIterator + Display + Sized + PartialEq {
    fn print_menu() {
        println!("What would you like to do?");
        for option in Self::iter().enumerate() {
            println!("{}. {}", option.0 + 1, option.1);
        }
    }

    fn from_input() -> Option<Self> {
        let mut input = String::new();

        print!("Please enter a command: ");
        io::stdout().flush().unwrap(); // Make sure the prompt is displayed immediately
        io::stdin().read_line(&mut input).ok()?; // Read a line of input

        parse_input(&input)
    }
}

#[derive(EnumIter, Display, Debug, PartialEq)]
enum CardMenuOptions {
    Create,
    List,
    Update,
    Delete,
    Quit,
}

impl MenuOptions for CardMenuOptions {}


#[derive(EnumIter, Display, Debug, PartialEq)]
enum MenuOption {
    // CardMenu(CardMenuOptions),
    CardMenu,
    Quit,
}

impl MenuOptions for MenuOption {}

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

async fn make_decision(tx: &mut Transaction<'_, Sqlite>, menu_choice: MenuOption) -> Result<bool, sqlx::Error> {
    match menu_choice {
        MenuOption::CardMenu => {
            CardMenuOptions::print_menu();
            let card_menu_choice = CardMenuOptions::from_input().unwrap();
            match card_menu_choice {
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
                CardMenuOptions::Quit => {
                    println!("Quitting");
                    return Ok(false)
                }
            }
        },
        MenuOption::Quit => {
            println!("Quitting");
            return Ok(false)
        }
    }
    println!("Done");
    Ok(true)
}

pub async fn start_app(tx: &mut Transaction<'_, Sqlite>) -> Result<(), sqlx::Error> {
    clear_screen();
    println!("Starting app");

    loop {
        println!("What would you like to do?");

        MenuOption::print_menu();

        let menu_choice = MenuOption::from_input().unwrap();

        println!("menu_choice: {:?}", menu_choice);

        if make_decision(tx, menu_choice).await? {
            // continue?
            println!("Press enter to continue");
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input = input.trim();
            if input != "" {
                break;
            }
            clear_screen();
        } else {
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
        assert_eq!(parse_input::<CardMenuOptions>("1"), Some(CardMenuOptions::Create));
        assert_eq!(parse_input::<CardMenuOptions>("2"), Some(CardMenuOptions::List));
        assert_eq!(parse_input::<CardMenuOptions>("3"), Some(CardMenuOptions::Update));
        assert_eq!(parse_input::<CardMenuOptions>("4"), Some(CardMenuOptions::Delete));
        assert_eq!(parse_input::<CardMenuOptions>("5"), Some(CardMenuOptions::Quit));
        assert_eq!(parse_input::<CardMenuOptions>("6"), None);
    }
}
