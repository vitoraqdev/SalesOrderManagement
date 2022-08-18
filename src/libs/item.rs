use super::super::schema::item;
use diesel::prelude::*;
use rocket::form::{Form, FromForm};
use rocket::response::status;
use crate::DATABASE_URL;


#[derive(Debug, Queryable)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub price: f64,
    pub description: String,
    pub is_active: bool,
}


#[derive(Debug, Insertable, FromForm)]
#[table_name = "item"]
pub struct NewItem {
    pub name: String,
    pub price: f64,
    pub description: String,
    pub is_active: bool,
}


#[get("/item/<item_id>")]
pub fn get_item_wrapper(item_id: i32) -> String {
    let conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let item = get_item(&conn, item_id);

    match item {
        Some(item) => format!("{:?}", item),
        None => "Item not found".to_string(),
    }
}

pub fn get_item(conn: &PgConnection, item_id: i32) -> Option<Item> {
    item::table
        .find(item_id)
        .first::<Item>(conn)
        .optional()
        .unwrap()
}

#[get("/item")]
pub fn get_all_items() -> Result<String, status::NotFound<String>> {
    let conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let items = _get_all_items(&conn);

    match items {
        Some(items) => Ok(format!("{:?}", items)),
        None => Err(status::NotFound("Items not found".to_string())),
    }
}

fn _get_all_items(conn: &PgConnection) -> Option<Vec<Item>> {
    item::table
        .load::<Item>(conn)
        .optional()
        .unwrap()
}

#[post("/item", data = "<item>")]
pub fn create_item(item: Form<NewItem>) -> String {
    let conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let item = diesel::insert_into(item::table)
        .values(item.into_inner())
        .get_result::<Item>(&conn)
        .expect("Error creating item");

    format!("{:?}", item)
}

#[delete("/item/<item_id>")]
pub fn delete_item(item_id: i32) -> Result<String, status::NotFound<String>> {
    let conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let result = _delete_item(&conn, item_id);

    match result {
        Ok(is_deleted) => {
            if is_deleted == 1 {
                Ok("Item deleted.".to_string())
            } else {
                Err(status::NotFound("Item not found.".to_string()))
            }
        }
        Err(e) => Err(status::NotFound(format!("Error deleting item: {}", e))),
    }
}

fn _delete_item(conn: &PgConnection, item_id: i32) -> QueryResult<usize> {
    diesel::delete(item::table.find(item_id))
        .execute(conn)
}