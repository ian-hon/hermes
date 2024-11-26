use sqlx::{Pool, Sqlite};

use crate::role::Role;

pub async fn user_permission_check(db: Pool<Sqlite>, user: String, channel: i32, p: Permissions) -> bool {
    match sqlx::query_as::<_, Role>("select * from roles where id in (select role_id from membership where user = $1 and channel_id = $2);")
        .bind(user)
        .bind(channel)
        .fetch_optional(&db)
        .await
        .unwrap() {
        Some(r) => permission_check(r.id, p),
        _ => false
    }
}

pub fn permission_check(content: i32, p: Permissions) -> bool {
    // not tested yet
    ((content >> (p as i32)) & 1) == 1
}

pub enum Permissions {
    ChannelDelete = 10,
    ChannelEdit = 9, // server name + description

    RoleCreate = 8,
    RoleDelete = 7,
    RoleEdit = 6, // name, colour, role permissions
    // only change perms of roles below own

    UserKick = 5,
    UserBan = 4,
    UserAdd = 3,

    MessageDelete = 2,
    MessageSend = 1
}
