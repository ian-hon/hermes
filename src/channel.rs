use sqlx::{Pool, Sqlite};

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
    pub async fn create(db: Pool<Sqlite>, name: String, description: String, creator: String) {
        sqlx::query("insert into channel(name, description) values($1, $2)")
            .bind(name)
            .bind(description)
            .execute(&db)
            .await.unwrap();

        // check if user exists
    }

    pub async fn delete(db: Pool<Sqlite>, name: String, description: String, creator: String) {
        
    }
}
