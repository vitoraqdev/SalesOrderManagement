use super::super::schema::address;
use super::super::schema::customer_address;
// use super::super::customer_address::dsl::*;
use diesel::prelude::*;
use rocket::form::{Form, FromForm};
use crate::{Address, DATABASE_URL};


#[derive(Debug, Queryable, Insertable, FromForm)]
#[table_name="customer_address"]
pub struct CustomerAddress {
    pub customer_id: i32,
    pub address_id: i32,
}


#[get("/customer_address/<customer_id>")]
pub fn get_customer_addresses_wrapper(customer_id: i32) -> String {
    let conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let addresses = get_customer_addresses(&conn, customer_id);

    format!("{:?}", addresses)
}

fn get_customer_addresses(conn: &PgConnection, customer_id: i32) -> Vec<Address> {
    let customer_addresses_id = customer_address::table
        .filter(customer_address::customer_id.eq(&customer_id))
        .load::<CustomerAddress>(conn)
        .optional()
        .unwrap()
        .unwrap()
        .iter()
        .map(|customer_address| customer_address.address_id)
        .collect::<Vec<i32>>();

    crate::get_addresses(conn, customer_addresses_id)
}

#[post("/customer_address", data = "<customer_address>")]
pub fn create_customer_address(customer_address: Form<CustomerAddress>) -> String {
    let conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let new_customer_address = diesel::insert_into(customer_address::table)
        .values(customer_address.into_inner())
        .get_result::<CustomerAddress>(&conn);

    match new_customer_address {
        Ok(customer_address) => format!("{:?}", customer_address),
        Err(e) => format!("Error creating customer address: {}", e),
    }
}
