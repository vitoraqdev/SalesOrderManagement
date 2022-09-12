use crate::libs::*;
use address::*;
use neighborhood::*;
use customer::*;
use customer_order::*;
use item::*;
use motoboy::*;
use order_details::*;

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            get_address, create_address, get_addresses, update_address, delete_address,
            get_customer, create_customer, get_customers, update_customer, delete_customer,
            get_order, get_orders, create_order, update_order, delete_order,
            get_item, create_item, get_all_items, update_item, delete_item,
            get_motoboy, create_motoboy, get_motoboys, update_motoboy, delete_motoboy,
            get_neighborhood, create_neighborhood, get_neighborhoods, update_neighborhood, delete_neighborhood,
            get_order_details, create_order_details, get_all_order_details, update_order_details, delete_order_details
        ])
}