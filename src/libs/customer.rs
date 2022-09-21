use crate::schema::customer;
use crate::DATABASE_URL;
use diesel::prelude::*;
use diesel::result::Error;
use rocket::form::{Form, FromForm};
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Serialize;


/// A Customer is an entity that can make orders. It has a name, a phone number and an address.
/// Phone is optional because it is not required to make an order, for example iFood does not
/// provide a phone number.
#[derive(Debug, Queryable, Serialize)]
pub struct Customer {
    /// Customer's database id
    pub id: i32,
    /// Customer's name
    pub name: String,
    /// Customer's phone number
    pub phone: Option<String>,
    /// Customer's address id
    pub address_id: i32,
}


/// A NewCustomer is a customer that is not yet in the database. It is used to create a new
/// customer. It has a name, a phone number and an address, but does *not* have an id, since it is
/// created automatically by the database.
#[derive(Insertable, AsChangeset, FromForm)]
#[diesel(table_name = customer)]
pub struct NewCustomer {
    /// Customer's name
    pub name: String,
    /// Customer's phone number
    pub phone: Option<String>,
    /// Customer's address id
    pub address_id: i32,
}

/// Returns a json of the customer by its id. If the customer is not found, returns a 404 status,
/// and any other error returns a 500 status.
///
/// # Example
/// $ curl GET http://localhost:8000/customer/1
/// {"id":1,"name":"John Doe","phone":"(11) 99999-9999","address_id":1}
#[get("/customer/<customer_id>")]
pub fn get_customer(customer_id: i32) -> Result<Json<Customer>, (Status, String)> {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let customer = _get_customer(&mut conn, customer_id);

    match customer {
        Ok(customer) => Ok(Json(customer)),
        Err(err) => match err {
            Error::NotFound => Err((Status::NotFound, "Customer not found".to_string())),
            Error::DatabaseError(_, info) => {
                Err((Status::InternalServerError, info.message().to_string()))
            }
            _ => Err((
                Status::InternalServerError,
                "Internal server error".to_string(),
            )),
        },
    }
}

/// Runs the query to get a customer by its id and Returns a QueryResult<Customer>.
pub fn _get_customer(conn: &mut PgConnection, customer_id: i32) -> QueryResult<Customer> {
    customer::table.find(customer_id).first::<Customer>(conn)
}

/// Creates a new customer in the database. If the customer is created successfully, returns a 200
/// status with the customer's id. If the customer is not created successfully, returns a 500
/// status.
///
/// # Example
/// $ curl POST localhost:8000/customer -d "name=John Doe&phone=(11) 99999-9999&address_id=1"
/// {"id":1, "name":"John Doe", "phone":"(11) 99999-9999", "address_id":1}
#[post("/customer", data = "<customer>")]
pub fn create_customer(customer: Form<NewCustomer>) -> Result<Json<Customer>, (Status, String)> {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let customer = _create_customer(&mut conn, customer.into_inner());

    match customer {
        Ok(customer) => Ok(Json(customer)),
        Err(err) => match err {
            Error::NotFound => Err((Status::NotFound, "Customer not found".to_string())),
            Error::DatabaseError(_, info) => {
                Err((Status::InternalServerError, info.message().to_string()))
            }
            _ => Err((
                Status::InternalServerError,
                "Internal server error".to_string(),
            )),
        },
    }
}

/// Creates a new customer in the database and returns a QueryResult<Customer>.
pub fn _create_customer(conn: &mut PgConnection, customer: NewCustomer) -> QueryResult<Customer> {
    diesel::insert_into(customer::table)
        .values(customer)
        .get_result::<Customer>(conn)
}

/// Returns a json of all customers in the database. Always returns a 200 status.
#[get("/customer")]
pub fn get_customers() -> Json<Vec<Customer>> {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let customers = _get_customers(&mut conn);

    Json(customers)
}

/// Runs the query to get all customers and returns a Vec<Customer>.
pub fn _get_customers(conn: &mut PgConnection) -> Vec<Customer> {
    customer::table.load::<Customer>(conn).unwrap()
}

/// Updates a customer in the database. If the customer is updated successfully, returns a 200
/// status with the customer's id. If the customer is not updated successfully, returns a 500
/// status. If the customer is not found, returns a 404 status.
///
/// # Example
/// $ curl PUT localhost:8000/customer/1 -d "name=Jane Doe&phone=(12) 8888-8888&address_id=2"
/// {"id":1, "name":"Jane Doe", "phone":"(12) 8888-8888", "address_id":2}
#[put("/customer/<customer_id>", data = "<customer>")]
pub fn update_customer(
    customer_id: i32,
    customer: Form<NewCustomer>,
) -> Result<Json<Customer>, (Status, String)> {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let customer = _update_customer(&mut conn, customer_id, customer.into_inner());

    match customer {
        Ok(customer) => Ok(Json(customer)),
        Err(err) => match err {
            Error::NotFound => Err((Status::NotFound, "Customer not found".to_string())),
            Error::DatabaseError(_, info) => {
                Err((Status::InternalServerError, info.message().to_string()))
            }
            _ => Err((
                Status::InternalServerError,
                "Internal Server Error".to_string(),
            )),
        },
    }
}

/// Runs a put query to update a customer and returns a QueryResult<Customer>.
pub fn _update_customer(
    conn: &mut PgConnection,
    customer_id: i32,
    customer: NewCustomer,
) -> QueryResult<Customer> {
    diesel::update(customer::table.find(customer_id))
        .set(customer)
        .get_result::<Customer>(conn)
}

/// Deletes a customer in the database. If the customer is deleted successfully, returns a 200
/// status. If the customer is not deleted successfully, returns a 500 status. If the customer is
/// not found, returns a 404 status.
///
/// # Example
/// $ curl -X DELETE localhost:8000/customer/1
/// {"id":1, "name":"Jane Doe", "phone":"(12) 8888-8888", "address_id":2}
#[delete("/customer/<customer_id>")]
pub fn delete_customer(customer_id: i32) -> Result<Status, (Status, String)> {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let customer = _delete_customer(&mut conn, customer_id);

    match customer {
        Ok(_) => Ok(Status::Ok),
        Err(err) => match err {
            Error::NotFound => Err((Status::NotFound, "Customer not found".to_string())),
            Error::DatabaseError(_, info) => {
                Err((Status::InternalServerError, info.message().to_string()))
            }
            _ => Err((
                Status::InternalServerError,
                "Internal Server Error".to_string(),
            )),
        },
    }
}

/// Runs a delete query to delete a customer and returns a QueryResult<usize>.
pub fn _delete_customer(conn: &mut PgConnection, customer_id: i32) -> QueryResult<usize> {
    diesel::delete(customer::table.find(customer_id)).execute(conn)
}
