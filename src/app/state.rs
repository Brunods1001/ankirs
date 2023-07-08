use crate::app::menus::MenuState;
use crate::models::User;
use colored::*;

#[derive(Debug, PartialEq, Clone)]
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
        // print navigation stack
        println!("{}", format!("Navigation stack {:?}", self.navigation_stack).green().bold());
        self.navigation_stack.pop();
        let previous_menu = self.navigation_stack.last().unwrap().clone();
        println!("{}", format!("Previous menu {:?}", previous_menu).green().bold());
        previous_menu
    }
}
