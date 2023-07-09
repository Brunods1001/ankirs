use bcrypt::{verify, hash, DEFAULT_COST};

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    username: String,
    is_authenticated: bool,
    is_guest: bool,
}

impl User {
    pub fn guest() -> Self {
        Self {
            username: "guest".to_string(),
            is_authenticated: false,
            is_guest: true,
        }
    }

    pub fn authenticate(&self, password: String) -> bool {
        let hashed_password = hash(password, DEFAULT_COST).unwrap();
        verify("password", &hashed_password).unwrap()
    }

    pub fn find_by_username(username: String) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            username,
            is_authenticated: false,
            is_guest: false,
        })
    }
}

pub struct Card {
    pub id: Option<i64>,
    pub front: String,
    pub back: String,
}

pub struct ListCard {
    pub id: i64,
    pub front: String,
    pub back: String,
}

pub struct ListDeck {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug)]
pub struct Deck {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
}
