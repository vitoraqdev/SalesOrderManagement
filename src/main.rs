
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

pub mod schema;
mod libs;

use diesel::{Connection, PgConnection};
use diesel::prelude::*;
use rocket::form::Form;
// use schema::neighborhood;


pub(crate) static DATABASE_URL: &str = "postgres://postgres:postgres@localhost:5432";


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![libs::neighborhood::get_neighborhood_wrapper,
            libs::neighborhood::create_neighborhood])
        .mount("/", routes![libs::address::get_address_wrapper,
            libs::address::create_address])
        .launch()
        .await?;

    Ok(())
}