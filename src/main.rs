
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
use libs::customer_address::*;
use libs::customer::*;

pub(crate) static DATABASE_URL: &str = "postgres://postgres:postgres@localhost:5432/salesordermanagement";


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![get_neighborhood_wrapper, create_neighborhood, get_neighborhoods, update_neighborhood, delete_neighborhood])
        .mount("/", routes![get_address, create_address, get_addresses, update_address, delete_address])
        .mount("/", routes![get_item_wrapper, create_item, update_item, delete_item, get_all_items])
        .mount("/", routes![get_motoboy, create_motoboy, update_motoboy, delete_motoboy, get_motoboys])
        .mount("/", routes![get_order_details_wrapper, create_order_details])
        .mount("/", routes![get_customer_addresses_wrapper, create_customer_address])
        .mount("/", routes![get_customer_wrapper, create_customer])
        .launch()
        .await?;

    Ok(())
}