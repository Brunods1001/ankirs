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
