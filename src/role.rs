use sqlx::prelude::FromRow;

#[derive(FromRow)]
pub struct Role {
    pub id: i32,
    pub channel_id: i32,

    pub name: String,
    pub colour: i32,

    pub content: i32,
    pub hiearchy: i32
}
