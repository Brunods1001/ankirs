use std::io;

use crate::models::{Card, Deck, ListCard, ListDeck};
use sqlx::{Acquire, SqlitePool};

use bcrypt::{hash, DEFAULT_COST};

pub async fn _create_user(
    pool: &SqlitePool,
    username: String,
    password: String,
) -> Result<i64, sqlx::Error> {
    println!("Creating user with username: {}", username);
    let mut tx = pool.begin().await?;
    let password_hash = hash(password, DEFAULT_COST).unwrap();
    let id = sqlx::query!(
        "INSERT INTO user (username, password_hash) VALUES (?, ?) RETURNING id;",
        username,
        password_hash
    )
    .fetch_one(tx.acquire().await?)
    .await?
    .id;

    tx.commit().await?;

    Ok(id)
}

pub async fn create_card(
    pool: &SqlitePool,
    front: String,
    back: String,
) -> Result<i64, sqlx::Error> {
    println!("Creating card with front: {}, back: {}", front, back);
    let mut tx = pool.begin().await?;
    let id = sqlx::query!(
        "INSERT INTO card (front, back) VALUES (?, ?) RETURNING id;",
        front,
        back
    )
    .fetch_one(tx.acquire().await?)
    .await?
    .id;
    tx.commit().await?;

    Ok(id)
}

pub async fn list_cards(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    let cards = sqlx::query_as!(ListCard, "SELECT id, front, back FROM card")
        .fetch_all(tx.acquire().await?)
        .await?;

    for card in cards {
        println!(
            "
            {}: | {} | {} |
            ",
            card.id, card.front, card.back
        );
    }

    Ok(())
}

pub async fn list_cards_for_deck(pool: &SqlitePool, deck_id: i64) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    let cards = sqlx::query_as!(
        ListCard,
        r#"
        SELECT id, front, back 
        FROM card 
        LEFT JOIN card_deck ON card.id = card_deck.card_id
        WHERE deck_id = ?
        "#,
        deck_id
    )
    .fetch_all(tx.acquire().await?)
    .await?;

    for card in cards {
        println!(
            "
            {}: | {} | {} |
            ",
            card.id, card.front, card.back
        );
    }

    Ok(())
}

pub async fn add_card_to_deck(
    pool: &SqlitePool,
    card_id: i64,
    deck_id: i64,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    println!(
        "Adding card with id {} to deck with id {}",
        card_id, deck_id
    );
    sqlx::query!(
        "INSERT INTO card_deck (card_id, deck_id) VALUES (?, ?)",
        card_id,
        deck_id
    )
    .execute(tx.acquire().await?)
    .await?;

    Ok(())
}

pub async fn update_card(
    pool: &SqlitePool,
    id: i64,
    front: Option<String>,
    back: Option<String>,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    println!("Updating card with id: {}", id);
    match (front, back) {
        (Some(front), Some(back)) => {
            sqlx::query!(
                "UPDATE card SET front = ?, back = ? WHERE id = ?",
                front,
                back,
                id
            )
            .execute(tx.acquire().await?)
            .await?;
        }
        (Some(front), None) => {
            sqlx::query!("UPDATE card SET front = ? WHERE id = ?", front, id)
                .execute(tx.acquire().await?)
                .await?;
        }
        (None, Some(back)) => {
            sqlx::query!("UPDATE card SET back = ? WHERE id = ?", back, id)
                .execute(tx.acquire().await?)
                .await?;
        }
        (None, None) => {
            println!("No changes to make");
        }
    }

    Ok(())
}

pub async fn get_deck_by_name(
    pool: &SqlitePool,
    name: String,
) -> Result<Option<Deck>, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let deck = sqlx::query_as!(
        Deck,
        "SELECT id, name, description FROM deck WHERE name = ?",
        name
    )
    .fetch_optional(tx.acquire().await?)
    .await?;

    Ok(deck)
}

pub async fn delete_deck_by_name(pool: &SqlitePool, name: String) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    println!("Deleting deck with name: {}", name);
    let res = sqlx::query!("DELETE FROM deck WHERE name = ?", name)
        .execute(tx.acquire().await?)
        .await?
        .rows_affected();

    tx.commit().await?;

    if res == 0 {
        println!("No deck with name {} found", name);
    }

    Ok(())
}

pub async fn create_deck(
    pool: &SqlitePool,
    name: String,
    description: Option<String>,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    println!("Creating deck with name: {}", name);
    sqlx::query!(
        "INSERT INTO deck (name, description) VALUES (?, ?)",
        name,
        description
    )
    .execute(tx.acquire().await?)
    .await?;

    Ok(())
}

pub async fn update_deck(
    pool: &SqlitePool,
    id: i64,
    name: String,
    description: Option<String>,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    println!("Creating deck with name: {}", name);
    sqlx::query!(
        "UPDATE deck SET name = ?, description = ? WHERE id = ?",
        name,
        description,
        id
    )
    .execute(tx.acquire().await?)
    .await?;

    Ok(())
}

pub async fn delete_deck(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    println!("Deleting deck with id: {}", id);
    let res = sqlx::query!("DELETE FROM deck WHERE id = ?", id)
        .execute(tx.acquire().await?)
        .await?
        .rows_affected();

    if res == 0 {
        println!("No deck with id: {} found", id);
    } else {
        println!("Deleted deck with id: {}", id);
    }

    Ok(())
}

pub async fn list_decks(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    let decks = sqlx::query_as!(ListDeck, "SELECT id, name, description FROM deck")
        .fetch_all(tx.acquire().await?)
        .await?;

    for deck in decks {
        println!(
            "
            {}: | {} | {} |
            ",
            deck.id,
            deck.name,
            deck.description.unwrap_or("".to_string())
        );
    }

    Ok(())
}

pub async fn delete_card(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    println!("Deleting card with id: {}", id);
    let res = sqlx::query!("DELETE FROM card WHERE id = ?", id)
        .execute(tx.acquire().await?)
        .await?
        .rows_affected();

    if res == 0 {
        println!("No card with id: {} found", id);
    } else {
        println!("Deleted card with id: {}", id);
    }

    Ok(())
}

pub async fn query_deck_exists(pool: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let res = sqlx::query!("SELECT id FROM deck WHERE id = ?", id)
        .fetch_optional(tx.acquire().await?)
        .await?;

    println!("Deck with id: {} exists: {}", id, res.is_some());
    Ok(res.is_some())
}

pub async fn query_deck_info(pool: &SqlitePool, id: i64) -> String {
    let mut tx = pool.begin().await.unwrap();
    let res = sqlx::query!("SELECT * FROM deck WHERE id = ?", id)
        .fetch_one(tx.acquire().await.unwrap())
        .await
        .unwrap();

    let (id, name, desc) = (res.id, res.name, res.description.unwrap_or("".to_string()));

    format!("{id}, {name}, {desc}").to_string()
}

pub async fn review_deck(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await.expect("Error starting transaction");
    let mut cards = sqlx::query_as!(
        Card,
        r#"
        SELECT id, front, back 
        FROM card 
        WHERE id IN (
            SELECT card_id 
            FROM card_deck 
            WHERE deck_id = ?)
        ORDER BY RANDOM();
        "#,
        id
    )
    .fetch_all(tx.acquire().await?)
    .await?;

    let mut correct = 0;
    let mut incorrect = 0;

    let mut tx_session = pool.begin().await.expect("Error starting transaction");
    // create a session
    let session_id = sqlx::query!("INSERT INTO session DEFAULT VALUES RETURNING id")
        .fetch_one(tx_session.acquire().await.unwrap())
        .await?
        .id;
    tx_session.commit().await?;
    let tx_session = pool.begin().await.expect("Error starting transaction");
    while !cards.is_empty() {
        let card = cards.pop().unwrap();
        println!("Front: {}", card.front);
        println!("What is the back?");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let answer = input.trim();
        // save answer
        let mut tx = pool.begin().await.expect("Error starting transaction");
        sqlx::query!(
            "INSERT INTO answer (session_id, card_id, answer, deck_id, correct_answer) VALUES (?, ?, ?, ?, ?);",
            session_id,
            card.id,
            answer,
            id,
            card.back,
        )
        .execute(tx.acquire().await?)
        .await?;
        tx.commit().await?;

        if answer == card.back {
            println!("Correct!");
            correct += 1;
        } else {
            println!("Incorrect!");
            println!("The answer is: {}", card.back);
            incorrect += 1;
            cards.push(card);
        }
    }

    tx_session.commit().await?;
    println!("You got {} correct and {} incorrect", correct, incorrect);

    Ok(())
}

pub async fn prompt_for_card_id_given_deck(
    pool: &SqlitePool,
    deck_id: i64,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    let cards = sqlx::query_as!(
        Card,
        r#"
        SELECT id, front, back 
        FROM card 
        WHERE id IN (
            SELECT card_id 
            FROM card_deck 
            WHERE deck_id = ?)
        ORDER BY RANDOM();
        "#,
        deck_id
    )
    .fetch_all(tx.acquire().await?)
    .await?;

    // print cards and ask the user to choose
    for card in cards {
        println!("{}: {}", card.id.unwrap(), card.front);
    }

    println!("Choose a card id: ");

    Ok(())
}

pub async fn query_card_by_id(pool: &SqlitePool, id: i64) -> Result<Card, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let card = sqlx::query_as!(
        Card,
        r#"
        SELECT id, front, back 
        FROM card 
        WHERE id = ?;
        "#,
        id
    )
    .fetch_one(tx.acquire().await?)
    .await?;

    Ok(card)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    async fn create_transaction() -> SqlitePool {
        dotenv().ok();
        let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = SqlitePool::connect(&database_url)
            .await
            .expect("Failed to create pool");
        pool
    }

    #[tokio::test]
    async fn test_create_card() {
        let pool = create_transaction().await;
        let mut tx = pool.begin().await.unwrap();

        create_card(&pool, "front".to_string(), "back".to_string())
            .await
            .unwrap();

        let card = sqlx::query!("SELECT front, back FROM card WHERE id = last_insert_rowid();")
            .fetch_one(tx.acquire().await.unwrap())
            .await
            .unwrap();

        assert_eq!(card.front, "front");
        assert_eq!(card.back, "back");

        tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_update_card() {
        let pool = create_transaction().await;
        let mut tx = pool.begin().await.unwrap();

        let record_id = sqlx::query!(
            "INSERT INTO card (front, back) VALUES (?, ?) RETURNING id",
            "front",
            "back"
        )
        .fetch_one(tx.acquire().await.unwrap())
        .await
        .unwrap()
        .id;

        update_card(&pool, record_id, Some("new front".to_string()), None)
            .await
            .unwrap();

        let card = sqlx::query!("SELECT front, back FROM card WHERE id = $1;", record_id)
            .fetch_one(tx.acquire().await.unwrap())
            .await
            .unwrap();

        assert_eq!(card.front, "new front");
        assert_eq!(card.back, "back");

        tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_delete_card() {
        let pool = create_transaction().await;
        let mut tx = pool.begin().await.unwrap();

        sqlx::query!(
            "INSERT INTO card (front, back) VALUES (?, ?)",
            "front",
            "back"
        )
        .execute(tx.acquire().await.unwrap())
        .await
        .unwrap();

        delete_card(&pool, 1).await.unwrap();

        let card = sqlx::query!("SELECT front, back FROM card WHERE id = 1;")
            .fetch_optional(tx.acquire().await.unwrap())
            .await
            .unwrap();

        assert_eq!(card.is_none(), true);

        tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_list_cards() {
        let pool = create_transaction().await;
        let mut tx = pool.begin().await.unwrap();

        sqlx::query!(
            "INSERT INTO card (front, back) VALUES (?, ?)",
            "front",
            "back"
        )
        .execute(tx.acquire().await.unwrap())
        .await
        .unwrap();

        sqlx::query!(
            "INSERT INTO card (front, back) VALUES (?, ?)",
            "front2",
            "back2"
        )
        .execute(tx.acquire().await.unwrap())
        .await
        .unwrap();

        list_cards(&pool).await.unwrap();

        tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_create_deck() {
        let pool = create_transaction().await;
        let mut tx = pool.begin().await.unwrap();

        create_deck(&pool, "deck".to_string(), Some("description".to_string()))
            .await
            .unwrap();

        let deck =
            sqlx::query!("SELECT name, description FROM deck WHERE id = last_insert_rowid();")
                .fetch_one(tx.acquire().await.unwrap())
                .await
                .unwrap();

        assert_eq!(deck.name, "deck");
        assert_eq!(deck.description, Some("description".to_string()));

        tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_list_decks() {
        let pool = create_transaction().await;
        let mut tx = pool.begin().await.unwrap();

        sqlx::query!(
            "INSERT INTO deck (name, description) VALUES (?, ?)",
            "deck",
            "description"
        )
        .execute(tx.acquire().await.unwrap())
        .await
        .unwrap();

        sqlx::query!(
            "INSERT INTO deck (name, description) VALUES (?, ?)",
            "deck2",
            "description2"
        )
        .execute(tx.acquire().await.unwrap())
        .await
        .unwrap();

        list_decks(&pool).await.unwrap();

        tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_unique_deck_names() {
        let pool = create_transaction().await;
        let mut tx = pool.begin().await.unwrap();

        create_deck(&pool, "deck".to_string(), Some("description".to_string()))
            .await
            .unwrap();

        let res = create_deck(&pool, "deck".to_string(), Some("description".to_string())).await;

        assert_eq!(res.is_err(), true);

        tx.rollback().await.unwrap();
    }
}
