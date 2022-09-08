use crate::schema::address;
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::form::{Form, FromForm};
use crate::DATABASE_URL;
use crate::libs::neighborhood::get_neighborhood;
use rocket::response::status;
use serde::Serialize;



#[derive(Debug, Queryable, Serialize)]
pub struct Address {
    pub id: i32,
    pub street: String,
    pub number: String,
    pub neighborhood_id: i32,
    pub complement: Option<String>,
    pub observation: Option<String>,
    pub delivery_fee: f64,
}

#[derive(Debug, AsChangeset, Insertable, FromForm)]
#[diesel(table_name = address)]
pub struct NewAddress {
    pub street: String,
    pub number: String,
    pub neighborhood_id: i32,
    pub complement: Option<String>,
    pub observation: Option<String>,
    pub delivery_fee: Option<f64>,
}

#[get("/address/<address_id>")]
pub fn get_address(address_id: i32) -> Result<Json<Address>, status::NotFound<String>> {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let address = _get_address(&mut conn, address_id);

    match address {
        Some(address) => Ok(Json(address)),
        None => Err(status::NotFound("Address not found".to_string())),
    }
}

fn _get_address(conn: &mut PgConnection, address_id: i32) -> Option<Address> {
    address::table
        .find(address_id)
        .first::<Address>(conn)
        .optional()
        .unwrap()
}

#[get("/address")]
pub fn get_addresses() -> Json<Vec<Address>> {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));
    let addresses = _get_addresses(&mut conn);
    Json(addresses)
}

fn _get_addresses(conn: &mut PgConnection) -> Vec<Address> {
    address::table
        .load::<Address>(conn)
        .unwrap()
}

#[post("/address", data = "<address>")]
pub fn create_address(mut address: Form<NewAddress>) -> Result<Json<Address>, status::BadRequest<String>> {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    // Check if delivery_fee form is empty
    // let mut address = address.clone();
    if address.delivery_fee.is_none() {
        // Set delivery_fee to neighborhood default value
        address.delivery_fee = Some(get_neighborhood(&mut conn, address.neighborhood_id)
            .unwrap()
            .delivery_fee);
    }

    let new_address = _create_address(&mut conn, address.into_inner());

    match new_address {
        Ok(address) => Ok(Json(address)),
        Err(_) => Err(status::BadRequest(Some("Error creating address".to_string()))),
    }
}

fn _create_address(conn: &mut PgConnection, address: NewAddress) -> QueryResult<Address> {
    diesel::insert_into(address::table)
        .values(address)
        .get_result::<Address>(conn)
}

#[put("/address/<address_id>", data = "<address>")]
pub fn update_address(address_id: i32, mut address: Form<NewAddress>) -> String {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));
    // Check if delivery_fee form is empty
    // let mut address = address.clone();
    if address.delivery_fee.is_none() {
        // Set delivery_fee to neighborhood default value
        address.delivery_fee = Some(get_neighborhood(&mut conn, address.neighborhood_id)
            .unwrap()
            .delivery_fee);
    }
    let updated_address = _update_address(&mut conn, address_id, address.into_inner());
    match updated_address {
        Ok(address) => format!("{:?}", address),
        Err(_) => "Error updating address".to_string(),
    }
}

fn _update_address(conn: &mut PgConnection, address_id: i32, address: NewAddress) -> QueryResult<Address> {
    diesel::update(address::table.find(address_id))
        .set(&address)
        .get_result::<Address>(conn)
}

#[delete("/address/<address_id>")]
pub fn delete_address(address_id: i32) -> String {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));
    let deleted_address = _delete_address(&mut conn, address_id);
    match deleted_address {
        Ok(_) => "Address deleted".to_string(),
        Err(_) => "Error deleting address".to_string(),
    }
}

fn _delete_address(conn: &mut PgConnection, address_id: i32) -> QueryResult<usize> {
    diesel::delete(address::table.find(address_id))
        .execute(conn)
}
