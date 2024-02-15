use diesel::Queryable;
use diesel::Insertable;
use super::schema::items;

#[derive(Queryable)]
pub struct Item {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "items"]
pub struct NewItem {
    pub name: String,
}
