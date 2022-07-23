use super::super::schema::order_details;
use diesel::prelude::*;
use rocket::form::{Form, FromForm};
use crate::DATABASE_URL;


#[derive(Debug, Queryable)]
pub struct OrderDetails {
    pub order_id: i32,
    pub item_id: i32,
    pub quantity: i32,
    pub unit_price: f64,
    pub total_price: f64,
}


#[derive(Debug, Insertable, FromForm)]
#[table_name="order_details"]
pub struct NewOrderDetails {
    pub order_id: i32,
    pub item_id: i32,
    pub quantity: i32,
    pub unit_price: f64,
}

#[get("/order_details/<order_id>")]
pub fn get_order_details_wrapper(order_id: i32) -> String {
    let conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let order_details = get_order_details(&conn, order_id);

    match order_details {
        Some(order_details) => format!("{:?}", order_details),
        None => "Order details not found".to_string(),
    }
}

pub fn get_order_details(conn: &PgConnection, order_id: i32) -> Option<OrderDetails> {
    order_details::table
        .filter(order_details::order_id.eq(order_id))
        .first::<OrderDetails>(conn)
        .optional()
        .unwrap()
}

#[post("/order_details", data = "<order_details>")]
pub fn create_order_details(order_details: Form<NewOrderDetails>) -> String {
    let conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let order_details = diesel::insert_into(order_details::table)
        .values(order_details.into_inner())
        .get_result::<OrderDetails>(&conn)
        .expect("Error creating order details");

    format!("{:?}", order_details)
}
