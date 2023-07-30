mod menus;
mod state;

use sqlx::SqlitePool;
use std::io::{self, Write};
use std::process::Command;

use crate::app::menus::traits::DecisionMaker;
use crate::app::menus::utils::prompt_for_deck_details;
use crate::app::state::AppState;
use crate::auth::login;
use crate::models::{Deck, User};
use crate::queries::{
    add_card_to_deck, create_card, create_deck, delete_deck_by_name, get_deck_by_name, list_decks,
    update_deck,
};
use crate::services::{CardService, DeckService};

use self::menus::utils::{prompt_for_card_details, prompt_for_card_id};

fn _clear_screen() {
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

pub async fn start_app(pool: SqlitePool) -> Result<(), sqlx::Error> {
    println!("Starting app");

    // log in
    let user: User = match login() {
        Ok(user) => user,
        Err(e) => {
            panic!("Error logging in {}", e);
        }
    };

    println!("Logged in as {:?}", user);

    let mut app_state = AppState::new(user);

    loop {
        let tx = pool.begin().await?;
        let (next_state, should_continue) = app_state
            .current_menu
            .make_decision(&pool, &app_state)
            .await?;
        app_state.navigate(next_state);

        if !should_continue {
            break;
        }

        tx.commit().await?;

        println!("State: {:?}", app_state);
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Menu {
    Main,
    Deck,
    DeckDetail(String), // deck name
    Quit,
}

impl Menu {
    fn prompt_user_to_choose_input(&self) -> Option<MenuOption> {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().parse::<usize>().unwrap();
        let options = self.get_options();
        if input > 0 && input <= options.len() {
            Some(options[input - 1].clone())
        } else {
            None
        }
    }

    fn get_options(&self) -> Vec<MenuOption> {
        match self {
            Menu::Main => vec![
                MenuOption {
                    name: "Deck".to_string(),
                    action: || Msg::Navigate(Menu::Deck),
                },
                MenuOption {
                    name: "Quit".to_string(),
                    action: || Msg::Quit,
                },
            ],
            Menu::Deck => vec![
                MenuOption {
                    name: "Create".to_string(),
                    action: || Msg::Deck(MsgDeck::Create),
                },
                MenuOption {
                    name: "View".to_string(),
                    action: || Msg::Deck(MsgDeck::View),
                },
                MenuOption {
                    name: "List".to_string(),
                    action: || Msg::Deck(MsgDeck::List),
                },
                MenuOption {
                    name: "Back".to_string(),
                    action: || Msg::Back,
                },
            ],
            Menu::DeckDetail(_) => vec![
                MenuOption {
                    name: "List".to_string(),
                    action: || Msg::Deck(MsgDeck::ListCards),
                },
                MenuOption {
                    name: "Update".to_string(),
                    action: || Msg::Deck(MsgDeck::Update),
                },
                MenuOption {
                    name: "Delete".to_string(),
                    action: || Msg::Deck(MsgDeck::Delete),
                },
                MenuOption {
                    name: "Add card to deck".to_string(),
                    action: || Msg::Deck(MsgDeck::AddCard),
                },
                MenuOption {
                    name: "Create card and add to deck".to_string(),
                    action: || Msg::Deck(MsgDeck::CreateCard),
                },
                MenuOption {
                    name: "Review".to_string(),
                    action: || Msg::Deck(MsgDeck::Review),
                },
                MenuOption {
                    name: "Report".to_string(),
                    action: || Msg::Deck(MsgDeck::Report),
                },
                MenuOption {
                    name: "Back".to_string(),
                    action: || Msg::Back,
                },
            ],
            Menu::Quit => vec![],
        }
    }

    fn view(&self) {
        println!("Menu: {:?}", self);
        for (i, option) in self.get_options().iter().enumerate() {
            println!("{}. {}", i + 1, option.name);
        }
    }
}

#[derive(Debug, Clone)]
struct MenuOption {
    name: String,
    // a function that returns a Msg
    action: fn() -> Msg,
}

#[derive(Debug)]
struct Model {
    current_menu: Menu,
    navigation_stack: Vec<Menu>,
    _user: Option<User>,
    deck: Option<Deck>,
}

#[derive(Debug, Clone)]
enum Msg {
    Navigate(Menu),
    Deck(MsgDeck),
    Stay,
    Back,
    Quit,
}

#[derive(Debug, Clone)]
enum MsgDeck {
    Create,
    Update,
    View,
    List,
    Delete,
    ListCards,
    ViewCard,
    AddCard,
    Review,
    Report,
    CreateCard,
}

impl Model {
    fn new() -> Self {
        let current_menu = Menu::Main;
        let navigation_stack = vec![];
        let user = None;
        let deck = None;

        Model {
            current_menu,
            navigation_stack,
            _user: user,
            deck,
        }
    }

    fn get_user_input(&self) -> Msg {
        match self.current_menu.prompt_user_to_choose_input() {
            Some(option) => (option.action)(),
            None => Msg::Stay,
        }
    }

    fn view(&self) {
        self.current_menu.view();
    }

    async fn update(&mut self, pool: &SqlitePool, msg: Msg) -> Result<(), String> {
        match msg {
            Msg::Navigate(menu) => {
                println!("Navigating to {:?}", menu);
                self.navigation_stack.push(self.current_menu.clone());
                self.current_menu = menu;
                Ok(())
            }
            Msg::Back => {
                println!("Going back");
                if let Some(menu) = self.navigation_stack.pop() {
                    self.current_menu = menu;
                }
                Ok(())
            }
            Msg::Stay => {
                println!("Staying in {:?}", self.current_menu);
                Ok(())
            }
            Msg::Quit => {
                println!("Quitting");
                Err("Quit".to_string())
            }
            Msg::Deck(msg) => handle_deck_msg(self, pool, msg).await,
        }
    }
}

async fn handle_deck_msg(model: &mut Model, pool: &SqlitePool, msg: MsgDeck) -> Result<(), String> {
    match msg {
        MsgDeck::Create => {
            println!("Creating deck");
            let (name, description) = prompt_for_deck_details().unwrap();
            create_deck(&pool, name, description)
                .await
                .expect("Error creating deck");
            Ok(())
        }
        MsgDeck::Update => {
            let tx = pool.begin().await.expect("Error starting transaction");
            let (name, description) = prompt_for_deck_details().unwrap();
            let id = model.deck.as_ref().expect("No deck set").id.unwrap();
            update_deck(&pool, id, name.clone(), description)
                .await
                .expect("Error updating deck");
            println!("Committing transaction");
            tx.commit().await.expect("Error committing transaction");

            let menu = Menu::DeckDetail(name);
            println!("Navigating to {:?}", menu);
            if let Some(last_menu) = model.navigation_stack.last() {
                if *last_menu != menu {
                    model.navigation_stack.push(menu.clone());
                    model.current_menu = menu;
                }
            };
            Ok(())
        }
        MsgDeck::View => {
            println!("Which deck would you like to view?");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_string();
            let tx = pool.begin().await.expect("Error starting transaction");
            let deck = get_deck_by_name(&pool, input.clone())
                .await
                .expect("Error getting deck")
                .expect("No deck found");
            println!("Deck: {:?}", deck);
            tx.commit().await.expect("Error committing transaction");

            let menu = Menu::DeckDetail(input);
            println!("Navigating to {:?}", menu);
            model.navigation_stack.push(menu.clone());
            model.current_menu = menu.clone();

            // set deck state
            println!("Setting model deck to {}", deck.name);
            model.deck = Some(deck);

            Ok(())
        }
        MsgDeck::Delete => {
            let tx = pool.begin().await.expect("Error starting transaction");
            delete_deck_by_name(
                &pool,
                model.deck.as_ref().expect("No deck found").name.clone(),
            )
            .await
            .expect("Error deleting deck");
            tx.commit().await.expect("Error committing transaction");

            // redirect to previous menu
            let menu = Menu::Deck;
            model.current_menu = menu;

            Ok(())
        }
        MsgDeck::List => {
            let tx = pool.begin().await.expect("Error starting transaction");
            list_decks(&pool).await.expect("Error listing decks");
            tx.commit().await.expect("Error committing transaction");
            Ok(())
        }
        MsgDeck::ListCards => {
            if let Some(deck) = &model.deck {
                println!("Listing cards for deck {}", deck.name);
                let res = DeckService::new(pool)
                    .list_cards(&deck.get_id())
                    .await
                    .expect("Error listing cards");
                Ok(res)
            } else {
                Err("No deck set".to_string())
            }
        }
        MsgDeck::ViewCard => {
            // prompt the user to choose a card from the list
            // this query needs to use cards associated with the deck
            if let Some(deck_id) = model.deck.as_ref().expect("No deck set").id {
                let card_id = DeckService::new(pool)
                    .prompt_for_card(deck_id)
                    .await
                    .expect("Error prompting for card");
                CardService::new(pool)
                    .view(card_id)
                    .await
                    .expect("Error prompting for card");
                Ok(())
            } else {
                Err("No deck set".to_string())
            }
        }
        MsgDeck::AddCard => {
            // get deck
            let deck = model.deck.as_ref().expect("No deck set");
            if let Some(deck_id) = deck.id {
                let tx = pool.begin().await.expect("Error starting transaction");
                let card_id = prompt_for_card_id().expect("Invalid input");
                add_card_to_deck(&pool, card_id, deck_id)
                    .await
                    .expect("Failed to add card to deck");
                tx.commit().await.expect("Error committing transaction");
            }
            Ok(())
        }
        MsgDeck::CreateCard => {
            // get deck
            let deck = model.deck.as_ref().expect("No deck set");
            if let Some(deck_id) = deck.id {
                let tx = pool.begin().await.expect("Error starting transaction");
                let (front, back) = prompt_for_card_details().expect("Invalid input");
                match create_card(
                    &pool,
                    front.unwrap_or("".to_string()),
                    back.unwrap_or("".to_string()),
                )
                .await
                {
                    Ok(card_id) => {
                        println!("Created card with id {card_id}");
                        match add_card_to_deck(&pool, card_id, deck_id).await {
                            Ok(()) => println!("Added card to deck"),
                            Err(e) => println!("Error: {}", e),
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                };
                tx.commit().await.expect("Error committing transaction");
            }
            Ok(())
        }
        MsgDeck::Review => {
            // reviews the cards in a deck
            let deck = model.deck.as_ref().expect("No deck set");
            if let Some(deck_id) = deck.id {
                DeckService::new(pool)
                    .review(deck_id)
                    .await
                    .expect("Error reviewing deck");

                println!("Try again?");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                if input.trim() == "y" {
                    println!("Try again");
                    Ok(())
                } else {
                    println!("Done");
                    Ok(())
                }
            } else {
                Err("No deck set".to_string())
            }
        }
        MsgDeck::Report => {
            // view the report for a deck
            let deck = model.deck.as_ref().expect("No deck set");
            if let Some(deck_id) = deck.id {
                DeckService::new(pool)
                    .view_report(deck_id)
                    .await
                    .expect("Error viewing report");
                Ok(())
            } else {
                Err("No deck set".to_string())
            }
        }
    }
}

pub async fn start_app_mvu(pool: SqlitePool) -> Result<(), sqlx::Error> {
    let mut model = Model::new();
    loop {
        model.view();
        let message = model.get_user_input();
        match model.update(&pool, message).await {
            Ok(_) => {
                println!("Updated model");
            }
            Err(e) => match e.as_str() {
                "Quit" => {
                    break;
                }
                _ => {
                    println!("Error updating model {}", e);
                    break;
                }
            },
        };
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_parse_input() {
        use crate::app::menus::{card::CardMenuOptions, utils::parse_input};
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
        assert_eq!(parse_input::<CardMenuOptions>("-1"), None);
    }
}
