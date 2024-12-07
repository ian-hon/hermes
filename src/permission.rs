use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, Row};

use crate::role::Role;

pub async fn user_permission_check(db: &Pool<Sqlite>, user: &String, channel: i32, p: Permissions) -> PermissionError {
    // check if user is server creator
    match sqlx::query("select count(*) from channel where id = $1 and creator = $2;")
        .bind(channel.clone())
        .bind(user.clone())
        .fetch_optional(db)
        .await.unwrap() {
        Some(i) => {
            if i.get::<i32, usize>(0) >= 1 {
                return PermissionError::Success;
            }
        },
        _ => {}
    }
    
    // regular perm check based on role
    match sqlx::query_as::<_, Role>("select * from roles where id in (select role_id from membership where user = $1 and channel_id = $2);")
        .bind(user)
        .bind(channel)
        .fetch_optional(db)
        .await
        .unwrap() {
        Some(r) => permission_check(r.id, p),
        _ => PermissionError::NoPermission
    }
}

pub fn generate_permission(p: Vec<Permissions>) -> i64 {
    let mut result = 0;
    for i in p.into_iter().map(|x| x as i64).collect::<HashSet<i64>>() {
        result |= 1 << i;
    }
    result
}

pub fn permission_check(content: i32, p: Permissions) -> PermissionError {
    if ((content >> (p as i32)) & 1) == 1 { PermissionError::Success } else { PermissionError::NoPermission }
}

pub enum Permissions {
    MessageView = 0,
    MessageSend = 1,
    MessageDelete = 2,

    UserAdd = 10,
    UserBan = 11,
    UserKick = 12,

    RoleEdit = 20, // name, colour, role permissions
    // only change perms of roles below own
    RoleDelete = 21,
    RoleCreate = 22,

    ChannelEdit = 30, // server name + description
    ChannelDelete = 31,
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum PermissionError {
    Success,
    NoPermission
}
