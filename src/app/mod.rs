mod menus;
mod state;

use sqlx::SqlitePool;
use std::io::{self, Write};
use std::process::Command;

use crate::app::menus::traits::DecisionMaker;
use crate::app::state::AppState;
use crate::auth::login;
use crate::models::User;
use crate::queries::{create_deck, list_decks, get_deck_by_name};

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
        let mut tx = pool.begin().await?;
        let (next_state, should_continue) = app_state
            .current_menu
            .make_decision(&mut tx, &app_state)
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

#[derive(Debug, Clone)]
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
                    action: || Msg::CreateDeck,
                },
                MenuOption {
                    name: "View".to_string(),
                    action: || Msg::ViewDeck,
                },
                MenuOption {
                    name: "List".to_string(),
                    action: || Msg::ListDecks,
                },
                MenuOption {
                    name: "Back".to_string(),
                    action: || Msg::Back,
                },
            ],
            Menu::DeckDetail(_) => vec![
                MenuOption {
                    name: "List".to_string(),
                    action: || Msg::ListDecks,
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
    user: Option<User>,
    model_pk: Option<i64>, // for detail menus
}

#[derive(Debug, Clone)]
enum Msg {
    Navigate(Menu),
    CreateDeck,
    ViewDeck,
    ListDecks,
    Stay,
    Input(String),
    Back,
    Quit,
}

impl Model {
    fn new() -> Self {
        let current_menu = Menu::Main;
        let navigation_stack = vec![];
        let user = None;
        let model_pk = None;

        Model {
            current_menu,
            navigation_stack,
            user,
            model_pk,
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
            Msg::Input(input) => {
                println!("Got input {}", input);
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
            Msg::CreateDeck => {
                println!("Creating deck");
                let name = "my MVU deck".to_string();
                let mut tx = pool.begin().await.expect("Error starting transaction");
                create_deck(&mut tx, name, None)
                    .await
                    .expect("Error creating deck");
                tx.commit().await.expect("Error committing transaction");
                Ok(())
            }
            Msg::ViewDeck => {
                println!("Which deck would you like to view?");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim().to_string();
                let mut tx = pool.begin().await.expect("Error starting transaction");
                let deck = get_deck_by_name(&mut tx, input.clone()).await.expect("Error getting deck");
                println!("Deck: {:?}", deck);
                tx.commit().await.expect("Error committing transaction");

                let menu = Menu::DeckDetail(input);
                println!("Navigating to {:?}", menu);
                self.navigation_stack.push(menu.clone());
                self.current_menu = menu.clone();


                Ok(())
            }
            Msg::ListDecks => {
                println!("Listing decks");
                let mut tx = pool.begin().await.expect("Error starting transaction");
                list_decks(&mut tx).await.expect("Error listing decks");
                tx.commit().await.expect("Error committing transaction");
                Ok(())
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
