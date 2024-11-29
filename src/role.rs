use sqlx::prelude::FromRow;

#[derive(FromRow)]
pub struct Role {
    pub id: i32,
    pub channel_id: i32,

    pub name: String,
    pub colour: i32,

    pub content: i64,
    pub hiearchy: i32
}

/*
create
delete
fetch
edit
    - change name
    - change colour
    - change perms
    - change hierarchy
*/

impl Role {
    
}
