use sqlx::types::time::PrimitiveDateTime;


pub struct ListCard {
    pub id: i64,
    pub front: String,
    pub back: String,
}
pub struct Card {
    front: String,
    back: String,
}

pub struct DbCard {
    id: i64,
    front: String,
    back: String,
    created_at: PrimitiveDateTime,
    updated_at: PrimitiveDateTime,
}
