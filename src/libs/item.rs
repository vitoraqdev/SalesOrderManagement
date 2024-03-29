use crate::schema::item;
use diesel::prelude::*;
use rocket::form::{Form, FromForm};
use rocket::response::status;
use crate::DATABASE_URL;
use serde::Serialize;
use rocket::serde::json::Json;


#[derive(Debug, Queryable, Serialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub price: f64,
    pub description: String,
    pub is_active: bool,
}


#[derive(Debug, AsChangeset, Insertable, FromForm)]
#[diesel(table_name = item)]
pub struct NewItem {
    pub name: String,
    pub price: f64,
    pub description: String,
    pub is_active: bool,
}


#[get("/item/<item_id>")]
pub fn get_item(item_id: i32) -> Result<Json<Item>, status::NotFound<String>> {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let item = _get_item(&mut conn, item_id);

    match item {
        Some(item) => Ok(Json(item)),
        None => Err(status::NotFound("Item not found".to_string())),
    }
}

pub fn _get_item(conn: &mut PgConnection, item_id: i32) -> Option<Item> {
    item::table
        .find(item_id)
        .first::<Item>(conn)
        .optional()
        .unwrap()
}

pub fn _get_item_price(conn: &mut PgConnection, item_id: i32) -> QueryResult<f64> {
    item::table
        .find(item_id)
        .select(item::price)
        .first::<f64>(conn)
}

#[get("/item")]
pub fn get_all_items() -> Result<Json<Vec<Item>>, status::NotFound<String>> {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let items = _get_all_items(&mut conn);

    match items {
        Some(items) => Ok(Json(items)),
        None => Err(status::NotFound("Items not found".to_string())),
    }
}

fn _get_all_items(conn: &mut PgConnection) -> Option<Vec<Item>> {
    item::table
        .load::<Item>(conn)
        .optional()
        .unwrap()
}

#[post("/item", data = "<item>")]
pub fn create_item(item: Form<NewItem>) -> String {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let item = diesel::insert_into(item::table)
        .values(item.into_inner())
        .get_result::<Item>(&mut conn)
        .expect("Error creating item");

    format!("{:?}", item)
}

#[put("/item/<item_id>", data = "<item>")]
pub fn update_item(item_id: i32, item: Form<NewItem>) -> String {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let item = _update_item(&mut conn, item_id, item.into_inner());

    format!("{:?}", item)
}

fn _update_item(conn: &mut PgConnection, item_id: i32, item: NewItem) -> QueryResult<Item> {
    diesel::update(item::table.find(item_id))
        .set(item)
        .get_result::<Item>(conn)
}

#[delete("/item/<item_id>")]
pub fn delete_item(item_id: i32) -> Result<String, status::NotFound<String>> {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let result = _delete_item(&mut conn, item_id);

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

fn _delete_item(conn: &mut PgConnection, item_id: i32) -> QueryResult<usize> {
    diesel::delete(item::table.find(item_id))
        .execute(conn)
}