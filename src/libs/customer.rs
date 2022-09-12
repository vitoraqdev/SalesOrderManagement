use crate::schema::customer;
use diesel::prelude::*;
use diesel::result::Error;
use rocket::form::{Form, FromForm};
use rocket::http::Status;
use crate::DATABASE_URL;
use serde::Serialize;
use rocket::serde::json::Json;


#[derive(Debug, Queryable, Serialize)]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub phone: Option<String>,
    pub address_id: i32,
}


#[derive(Insertable, AsChangeset, FromForm)]
#[diesel(table_name = customer)]
pub struct NewCustomer {
    pub name: String,
    pub phone: Option<String>,
    pub address_id: i32,
}


#[get("/customer/<customer_id>")]
pub fn get_customer(customer_id: i32) -> Result<Json<Customer>, (Status, String)> {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let customer = _get_customer(&mut conn, customer_id);

    match customer {
        Ok(customer) => Ok(Json(customer)),
        Err(err) => match err {
            Error::NotFound => Err((Status::NotFound, "Customer not found".to_string())),
            Error::DatabaseError(_, info) => Err((Status::InternalServerError, info.message().to_string())),
            _ => Err((Status::InternalServerError, "Internal server error".to_string())),
        }
    }
}

pub fn _get_customer(conn: &mut PgConnection, customer_id: i32) -> QueryResult<Customer> {
    customer::table
        .find(customer_id)
        .first::<Customer>(conn)
}

#[post("/customer", data = "<customer>")]
pub fn create_customer(customer: Form<NewCustomer>) -> String {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let customer = diesel::insert_into(customer::table)
        .values(customer.into_inner())
        .get_result::<Customer>(&mut conn);

    match customer {
        Ok(customer) => format!("{:?}", customer),
        Err(e) => format!("Error creating customer: {}", e),
    }
}

#[get("/customer")]
pub fn get_customers() -> Json<Vec<Customer>> {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let customers = _get_customers(&mut conn);

    Json(customers)
}

pub fn _get_customers(conn: &mut PgConnection) -> Vec<Customer> {
    customer::table
        .load::<Customer>(conn)
        .unwrap()
}

#[put("/customer/<customer_id>", data = "<customer>")]
pub fn update_customer(customer_id: i32, customer: Form<NewCustomer>) -> Result<Json<Customer>, (Status, String)> {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let customer = _update_customer(&mut conn, customer_id, customer.into_inner());

    match customer {
        Ok(customer) => Ok(Json(customer)),
        Err(err) => match err {
            Error::NotFound => Err((Status::NotFound, "Customer not found".to_string())),
            Error::DatabaseError(_, info) => Err((Status::InternalServerError, info.message().to_string())),
            _ => Err((Status::InternalServerError, "Internal Server Error".to_string())),
        }
    }
}

pub fn _update_customer(conn: &mut PgConnection, customer_id: i32, customer: NewCustomer) -> QueryResult<Customer> {
    diesel::update(customer::table.find(customer_id))
        .set(customer)
        .get_result::<Customer>(conn)
}

#[delete("/customer/<customer_id>")]
pub fn delete_customer(customer_id: i32) -> Result<Status, (Status, String)> {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let customer = _delete_customer(&mut conn, customer_id);

    match customer {
        Ok(_) => Ok(Status::Ok),
        Err(err) => match err {
            Error::NotFound => Err((Status::NotFound, "Customer not found".to_string())),
            Error::DatabaseError(_, info) => Err((Status::InternalServerError, info.message().to_string())),
            _ => Err((Status::InternalServerError, "Internal Server Error".to_string())),
        }
    }
}

pub fn _delete_customer(conn: &mut PgConnection, customer_id: i32) -> QueryResult<usize> {
    diesel::delete(customer::table.find(customer_id))
        .execute(conn)
}
