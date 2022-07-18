use super::super::schema::address;
use diesel::prelude::*;
use rocket::form::{Form, FromForm};
use crate::DATABASE_URL;
use crate::libs::neighborhood::get_neighborhood;


#[derive(Debug, Queryable)]
pub struct Address {
    pub id: i32,
    pub street: String,
    pub number: String,
    pub neighborhood_id: i32,
    pub complement: Option<String>,
    pub observation: Option<String>,
    pub delivery_fee: f64,
}

#[derive(Insertable, FromForm)]
#[table_name="address"]
pub struct NewAddress {
    pub street: String,
    pub number: String,
    pub neighborhood_id: i32,
    pub complement: Option<String>,
    pub observation: Option<String>,
    pub delivery_fee: Option<f64>,
}

#[get("/address/<address_id>")]
pub fn get_address_wrapper(address_id: i32) -> String {
    let conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let address = get_address(&conn, address_id);

    match address {
        Some(address) => format!("{:?}", address),
        None => "Address not found".to_string(),
    }
}

fn get_address(conn: &PgConnection, address_id: i32) -> Option<Address> {
    address::table
        .find(address_id)
        .first::<Address>(conn)
        .optional()
        .unwrap()
}

#[post("/address", data = "<address>")]
pub fn create_address(mut address: Form<NewAddress>) -> String {
    let conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    // Check if delivery_fee form is empty
    // let mut address = address.clone();
    if address.delivery_fee.is_none() {
        // Set delivery_fee to neighborhood default value
        address.delivery_fee = Some(get_neighborhood(&conn, address.neighborhood_id)
            .unwrap()
            .delivery_fee);
    }

    let new_address = diesel::insert_into(address::table)
        .values(address.into_inner())
        .get_result::<Address>(&conn);
    match new_address {
        Ok(address) => format!("{:?}", address),
        Err(_) => "Error creating address".to_string(),
    }
}