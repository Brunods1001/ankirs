use crate::app::menus::MenuState;

#[derive(Debug, Clone, PartialEq)]
pub struct AppState {
    pub current_menu: MenuState,
    navigation_stack: Vec<MenuState>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        Self {
            current_menu: MenuState::MainMenu,
            navigation_stack: vec![MenuState::MainMenu],
        }
    }

    pub fn navigate(&mut self, new_menu: MenuState) {
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

