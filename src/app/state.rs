use crate::app::menus::MenuState;
use crate::models::User;
use colored::*;

#[derive(Debug, Clone, PartialEq)]
pub struct AppState {
    pub current_menu: MenuState,
    navigation_stack: Vec<MenuState>,
    user: User
}

impl Default for AppState {
    fn default() -> Self {
        Self::new(User::guest())
    }
}

impl AppState {
    pub fn new(user: User) -> Self {
        Self {
            current_menu: MenuState::MainMenu,
            navigation_stack: vec![MenuState::MainMenu],
            user
        }
    }

    pub fn navigate(&mut self, new_menu: MenuState) {
        println!("{}", format!("Navigating to {:?}", new_menu).red().bold());
        if self.current_menu == new_menu {
            return;
        }
        self.navigation_stack.push(new_menu);
        self.current_menu = new_menu;
    }

    pub fn get_previous_menu(&mut self) -> MenuState {
        self.navigation_stack.pop().unwrap()
    }
}
