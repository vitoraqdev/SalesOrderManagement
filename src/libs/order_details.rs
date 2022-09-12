use crate::schema::order_details;
use diesel::prelude::*;
use diesel::result::Error;
use rocket::form::{Form, FromForm};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use crate::DATABASE_URL;

#[derive(Debug, Queryable, Serialize)]
pub struct OrderDetails {
    pub order_id: i32,
    pub item_id: i32,
    pub quantity: i32,
    pub unit_price: f64,
    pub total_price: f64,
}


#[derive(Debug, AsChangeset, Insertable, FromForm)]
#[diesel(table_name = order_details)]
pub struct NewOrderDetails {
    pub order_id: i32,
    pub item_id: i32,
    pub quantity: i32,
    #[field(default = 0.0)] // later defined on create_order_details
    pub unit_price: f64, // maybe get from database? no reason to manually input unit price
    // total price should be calculated automatically on the database side
}

#[get("/order_details/<order_id>")]
pub fn get_order_details(order_id: i32) -> Result<Json<Vec<OrderDetails>>, (Status, String)> {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let order_details = _get_order_details(&mut conn, order_id);

    match order_details {
        Ok(order_details) => Ok(Json(order_details)),
        Err(err) => match err {
            Error::NotFound => Err((Status::NotFound, "Order details not found".to_string())),
            Error::DatabaseError(_, info) => Err((Status::InternalServerError, info.message().to_string())),
            _ => Err((Status::InternalServerError, "Internal server error".to_string())),
        }
    }
}

pub fn _get_order_details(conn: &mut PgConnection, order_id: i32) -> QueryResult<Vec<OrderDetails>> {
    order_details::table
        .filter(order_details::order_id.eq(order_id))
        .load::<OrderDetails>(conn)
}

#[post("/order_details", data = "<order_details>")]
pub fn create_order_details(mut order_details: Form<NewOrderDetails>) -> Result<Json<OrderDetails>, (Status, String)> {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    // get unit price from database
    let unit_price = crate::libs::item::_get_item_price(&mut conn, order_details.item_id);

    // calculate total price
    match unit_price {
        Ok(unit_price) => {
            order_details.unit_price = unit_price;

            let order_details = _create_order_details(&mut conn, order_details.into_inner());

            match order_details {
                Ok(order_details) => Ok(Json(order_details)),
                Err(Error::NotFound) => Err((Status::NotFound, "Order details not found".to_string())),
                Err(Error::DatabaseError(_, information)) => Err((Status::InternalServerError, information.message().to_string())),
                Err(err) => Err((Status::InternalServerError, err.to_string())),
            }
        },
        // match unit_price errors
        Err(Error::NotFound) => Err((Status::NotFound, "Item not found".to_string())),
        Err(Error::DatabaseError(_, information)) => Err((Status::InternalServerError, information.message().to_string())),
        Err(err) => Err((Status::InternalServerError, err.to_string())),
    }
}

pub fn _create_order_details(conn: &mut PgConnection, order_details: NewOrderDetails) -> QueryResult<OrderDetails> {
    diesel::insert_into(order_details::table)
        .values(order_details)
        .get_result::<OrderDetails>(conn)
}

#[get("/order_details")]
pub fn get_all_order_details() -> Result<Json<Vec<OrderDetails>>, Status> {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let order_details = _get_all_order_details(&mut conn);

    match order_details {
        Ok(order_details) => Ok(Json(order_details)),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub fn _get_all_order_details(conn: &mut PgConnection) -> QueryResult<Vec<OrderDetails>> {
    order_details::table
        .load::<OrderDetails>(conn)
}

#[put("/order_details/<order_id>", data = "<order_details>")]
pub fn update_order_details(order_id: i32, order_details: Form<NewOrderDetails>) -> Result<Json<OrderDetails>, (Status, String)> {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let order_details = _update_order_details(&mut conn, order_id, order_details.into_inner());

    match order_details {
        Ok(order_details) => Ok(Json(order_details)),
        Err(err) => match err {
            Error::NotFound => Err((Status::NotFound, "Order id not found.".to_string())),
            err => Err((Status::InternalServerError, err.to_string()))
        }
    }
}

pub fn _update_order_details(conn: &mut PgConnection, order_id: i32, order_details: NewOrderDetails) -> QueryResult<OrderDetails> {
    diesel::update(order_details::table)
        .filter(order_details::order_id.eq(order_id))
        .set(order_details)
        .get_result::<OrderDetails>(conn)
}

#[delete("/order_details/<order_id>")]
pub fn delete_order_details(order_id: i32) -> Result<Json<OrderDetails>, (Status, String)> {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let order_details = _delete_order_details(&mut conn, order_id);

    match order_details {
        Ok(order_details) => Ok(Json(order_details)),
        Err(err) => match err {
            Error::NotFound => { Err((Status::NotFound, "Order id not found.".to_string())) }
            err => { Err((Status::InternalServerError, err.to_string())) }
        }
    }
}

pub fn _delete_order_details(conn: &mut PgConnection, order_id: i32) -> QueryResult<OrderDetails> {
    diesel::delete(order_details::table)
        .filter(order_details::order_id.eq(order_id))
        .get_result::<OrderDetails>(conn)
}