use crate::schema::customer_order;
use diesel::prelude::*;
use diesel::result::Error;
use rocket::serde::json::Json;
use rocket::form::{Form, FromForm};
use rocket::http::Status;
use crate::DATABASE_URL;
use serde::Serialize;
use rocket::time::Date;


#[derive(Debug, Queryable, Serialize)]
pub struct CustomerOrder {
    pub id: i32,
    pub date: Date,
    pub customer_id: i32,
    pub motoboy_id: Option<i32>,
    pub address_id: Option<i32>,
    pub source: i16, // change to platform
    pub additional: f64,
    pub delivery_fee: f64,
    pub discount: f64,
    pub status: i16,
}

#[derive(Debug, AsChangeset, Insertable, FromForm)]
#[diesel(table_name = customer_order)]
pub struct NewCustomerOrder {
    pub date: Date,
    pub customer_id: i32,
    pub motoboy_id: Option<i32>,
    pub address_id: Option<i32>,
    pub source: i16, // change to platform
    pub additional: f64,
    pub delivery_fee: f64,
    pub discount: f64,
    pub status: i16,
}

#[get("/order/<order_id>")]
pub fn get_order(order_id: i32) -> Result<Json<CustomerOrder>, Status> {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let address = _get_order(&mut conn, order_id);

    match address {
        Ok(address) => Ok(Json(address)),
        Err(err) => match err {
            Error::NotFound => Err(Status::NotFound),
            _ => Err(Status::InternalServerError),
        }
    }
}

fn _get_order(conn: &mut PgConnection, order_id: i32) -> QueryResult<CustomerOrder> {
    customer_order::table
        .find(order_id)
        .first::<CustomerOrder>(conn)
}

#[get("/order")]
pub fn get_orders() -> Json<Vec<CustomerOrder>> {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));
    let orders = _get_orders(&mut conn);
    Json(orders)
}

fn _get_orders(conn: &mut PgConnection) -> Vec<CustomerOrder> {
    customer_order::table
        .load::<CustomerOrder>(conn)
        .unwrap()
}

#[post("/order", data = "<order>")]
pub fn create_order(order: Form<NewCustomerOrder>) -> Result<Json<CustomerOrder>, Status> {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let new_order = _create_order(&mut conn, order.into_inner());

    match new_order {
        Ok(order) => Ok(Json(order)),
        Err(err) => Err(Status::InternalServerError),
    }
}

fn _create_order(conn: &mut PgConnection, order: NewCustomerOrder) -> QueryResult<CustomerOrder> {
    diesel::insert_into(customer_order::table)
        .values(order)
        .get_result::<CustomerOrder>(conn)
}

#[put("/order/<order_id>", data = "<order>")]
pub fn update_order(order_id: i32, mut order: Form<NewCustomerOrder>) -> Result<Json<CustomerOrder>, Status> {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let updated_order = _update_order(&mut conn, order_id, order.into_inner());

    match updated_order {
        Ok(order) => Ok(Json(order)),
        Err(err) => match err {
            Error::NotFound => Err(Status::NotFound),
            _ => Err(Status::InternalServerError),
        }
    }
}

fn _update_order(conn: &mut PgConnection, order_id: i32, order: NewCustomerOrder) -> QueryResult<CustomerOrder> {
    diesel::update(customer_order::table.find(order_id))
        .set(&order)
        .get_result::<CustomerOrder>(conn)
}

#[delete("/order/<order_id>")]
pub fn delete_order(order_id: i32) -> String {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let deleted_order = _delete_order(&mut conn, order_id);

    match deleted_order {
        Ok(_) => "Order deleted".to_string(),
        Err(_) => "Error deleting order".to_string(),
    }
}

fn _delete_order(conn: &mut PgConnection, order_id: i32) -> QueryResult<usize> {
    diesel::delete(customer_order::table.find(order_id))
        .execute(conn)
}
