use sqlx::{Pool, Sqlite};

use crate::permission::{user_permission_check, Permissions};

pub struct Channel {
    id: i32,
    name: String,
    description: String
}
// create
// delete
// edit
// fetch

// join
// leave
// add
impl Channel {
    pub async fn create(db: Pool<Sqlite>, name: String, description: String, user: String) {
        sqlx::query("insert into channel(name, description) values($1, $2)")
            .bind(name)
            .bind(description)
            .execute(&db)
            .await.unwrap();

        // check if user exists
    }

    pub async fn delete(db: Pool<Sqlite>, channel: i32, user: String) {
        if !user_permission_check(db, user, channel, Permissions::ChannelDelete).await {
            
        }
    }
}
