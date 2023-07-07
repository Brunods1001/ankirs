use crate::models::{ListCard, ListDeck};
use sqlx::{Acquire, Sqlite, Transaction};

pub async fn create_card(
    tx: &mut Transaction<'_, Sqlite>,
    front: String,
    back: String,
) -> Result<(), sqlx::Error> {
    println!("Creating card with front: {}, back: {}", front, back);
    sqlx::query!("INSERT INTO card (front, back) VALUES (?, ?)", front, back)
        .execute(tx.acquire().await?)
        .await?;

    Ok(())
}

pub async fn list_cards(tx: &mut Transaction<'_, Sqlite>) -> Result<(), sqlx::Error> {
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

pub async fn update_card(
    tx: &mut Transaction<'_, Sqlite>,
    id: i64,
    front: Option<String>,
    back: Option<String>,
) -> Result<(), sqlx::Error> {
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


pub async fn create_deck(
    tx: &mut Transaction<'_, Sqlite>,
    name: String,
    description: Option<String>,
) -> Result<(), sqlx::Error> {
    println!("Creating deck with name: {}", name);
    sqlx::query!("INSERT INTO deck (name, description) VALUES (?, ?)", name, description)
        .execute(tx.acquire().await?)
        .await?;

    Ok(())
}

pub async fn list_decks(
    tx: &mut Transaction<'_, Sqlite>,
) -> Result<(), sqlx::Error> {
    let decks = sqlx::query_as!(ListDeck, "SELECT id, name, description FROM deck")
        .fetch_all(tx.acquire().await?)
        .await?;

    for deck in decks {
        println!(
            "
            {}: | {} | {} |
            ",
            deck.id, deck.name, deck.description.unwrap_or("".to_string())
        );
    }

    Ok(())
}

pub async fn delete_card(tx: &mut Transaction<'_, Sqlite>, id: i64) -> Result<(), sqlx::Error> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    async fn create_transaction() -> Transaction<'static, Sqlite> {
        dotenv().ok();
        let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn = sqlx::sqlite::SqlitePoolOptions::new()
            .connect(&database_url)
            .await
            .unwrap();

        conn.begin().await.unwrap()
    }

    #[tokio::test]
    async fn test_create_card() {
        let mut tx = create_transaction().await;

        create_card(&mut tx, "front".to_string(), "back".to_string())
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
        let mut tx = create_transaction().await;

        let record_id = sqlx::query!("INSERT INTO card (front, back) VALUES (?, ?) RETURNING id", "front", "back")
            .fetch_one(tx.acquire().await.unwrap())
            .await
            .unwrap()
            .id;


        update_card(&mut tx, record_id, Some("new front".to_string()), None)
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
        let mut tx = create_transaction().await;

        sqlx::query!("INSERT INTO card (front, back) VALUES (?, ?)", "front", "back")
            .execute(tx.acquire().await.unwrap())
            .await
            .unwrap();

        delete_card(&mut tx, 1).await.unwrap();

        let card = sqlx::query!("SELECT front, back FROM card WHERE id = 1;")
            .fetch_optional(tx.acquire().await.unwrap())
            .await
            .unwrap();

        assert_eq!(card.is_none(), true);

        tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_list_cards() {
        let mut tx = create_transaction().await;

        sqlx::query!("INSERT INTO card (front, back) VALUES (?, ?)", "front", "back")
            .execute(tx.acquire().await.unwrap())
            .await
            .unwrap();

        sqlx::query!("INSERT INTO card (front, back) VALUES (?, ?)", "front2", "back2")
            .execute(tx.acquire().await.unwrap())
            .await
            .unwrap();

        list_cards(&mut tx).await.unwrap();

        tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_create_deck() {
        let mut tx = create_transaction().await;

        create_deck(&mut tx, "deck".to_string(), Some("description".to_string()))
            .await
            .unwrap();

        let deck = sqlx::query!("SELECT name, description FROM deck WHERE id = last_insert_rowid();")
            .fetch_one(tx.acquire().await.unwrap())
            .await
            .unwrap();

        assert_eq!(deck.name, "deck");
        assert_eq!(deck.description, Some("description".to_string()));

        tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_list_decks() {
        let mut tx = create_transaction().await;

        sqlx::query!("INSERT INTO deck (name, description) VALUES (?, ?)", "deck", "description")
            .execute(tx.acquire().await.unwrap())
            .await
            .unwrap();

        sqlx::query!("INSERT INTO deck (name, description) VALUES (?, ?)", "deck2", "description2")
            .execute(tx.acquire().await.unwrap())
            .await
            .unwrap();

        list_decks(&mut tx).await.unwrap();

        tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_unique_deck_names() {
        let mut tx = create_transaction().await;

        create_deck(&mut tx, "deck".to_string(), Some("description".to_string()))
            .await
            .unwrap();

        let res = create_deck(&mut tx, "deck".to_string(), Some("description".to_string()))
            .await;

        assert_eq!(res.is_err(), true);

        tx.rollback().await.unwrap();
    }
}
