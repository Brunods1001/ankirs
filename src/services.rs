use sqlx::SqlitePool;

use crate::queries::{list_cards_for_deck, query_card_by_id, review_deck};

pub struct CardService<'a> {
    pool: &'a SqlitePool,
}
pub struct DeckService<'a> {
    pool: &'a SqlitePool,
}

impl<'a> CardService<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        CardService { pool }
    }

    pub fn list(&self) {
        println!("list cards");
    }

    pub fn list_by_deck(&self, deck_id: i64) {
        println!("list cards by deck");
    }

    pub fn create(&self, front: String, back: String) {
        println!("create card");
    }

    pub fn update(&self, id: i64, front: Option<String>, back: Option<String>) {
        println!("update card");
    }

    pub async fn view(&self, card_id: i64) -> Result<(), sqlx::Error> {
        println!("View a card");
        match query_card_by_id(self.pool, card_id).await {
            Ok(card) => {
                println!("Card: {:?}", card);
                Ok(())
            }
            Err(e) => {
                println!("Error: {:?}", e);
                Err(e)
            }
        }
    }
}

impl<'a> DeckService<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        DeckService { pool }
    }
    pub async fn list_cards(&self, deck_id: &i64) -> Result<(), sqlx::Error> {
        list_cards_for_deck(self.pool, deck_id)
            .await
            .expect("Error listing cards for deck");
        Ok(())
    }

    pub async fn prompt_for_card(&self, deck_id: i64) -> Result<i64, sqlx::Error> {
        println!("Choose a card");
        let mut card_id = String::new();
        list_cards_for_deck(self.pool, &deck_id)
            .await
            .expect("Error listing cards for deck");
        // get card id from user
        std::io::stdin()
            .read_line(&mut card_id)
            .expect("Failed to read line");
        // parse card id to i64
        let card_id: i64 = card_id.trim().parse().expect("Please type a number!");
        Ok(card_id)
    }

    pub async fn review(&self, deck_id: i64) -> Result<(), sqlx::Error> {
        let res = review_deck(self.pool, deck_id).await;
        res
    }

    pub async fn view_report(&self, deck_id: i64) -> Result<(), sqlx::Error> {
        println!("Viewing deck with id {}", deck_id);
        // get sessions and answers related to deck
        let res = sqlx::query!(
            r#"
            SELECT
                a.deck_id,
                a.id AS answer_id,
                a.card_id,
                a.session_id,
                a.correct_answer,
                a.time AS answer_time
            FROM answer a
            LEFT JOIN session s ON a.session_id = s.id
            WHERE a.deck_id = ?
            "#,
            deck_id
        );   
        for row in res.fetch_all(self.pool).await? {
            println!("Answer: {:?}", row);
        }
        Ok(())
    }
}
