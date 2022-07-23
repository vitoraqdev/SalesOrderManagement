
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

pub mod schema;
mod libs;

// use schema::neighborhood;
use libs::neighborhood::*;
use libs::address::*;
use libs::item::*;
use libs::motoboy::*;
use libs::order_details::*;

pub(crate) static DATABASE_URL: &str = "postgres://postgres:postgres@localhost:5432";


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![get_neighborhood_wrapper, create_neighborhood])
        .mount("/", routes![get_address_wrapper, create_address])
        .mount("/", routes![get_item_wrapper, create_item])
        .mount("/", routes![get_motoboy_wrapper, create_motoboy])
        .mount("/", routes![get_order_details_wrapper, create_order_details])
        .launch()
        .await?;

    Ok(())
}