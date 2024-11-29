use sqlx::{prelude::FromRow, Pool, Sqlite};

#[derive(FromRow)]
pub struct Membership {
    pub id: i32,
    pub channel_id: i32,
    pub user: String,
    pub role_id: i32 // -1 represents no role (default)
}
impl Membership {
    pub async fn add_membership(db: &Pool<Sqlite>, user: String, channel_id: i32, role_id: i32) {

    }

    pub async fn fetch_membership(db: &Pool<Sqlite>, user: String, channel_id: i32) -> Option<Membership> {
        sqlx::query_as::<_, Membership>("select * from membership where user = $1 and channel_id = $2;")
            .bind(user)
            .bind(channel_id)
            .fetch_optional(db)
            .await.unwrap()
    }

    pub async fn remove_membership() {

    }
}
