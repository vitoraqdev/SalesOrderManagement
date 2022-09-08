use crate::schema::customer;
use diesel::prelude::*;
use rocket::form::{Form, FromForm};
use crate::DATABASE_URL;


#[derive(Debug, Queryable)]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub phone: Option<String>,
    pub address_id: i32,
}


#[derive(Insertable, FromForm)]
#[diesel(table_name = customer)]
pub struct NewCustomer {
    pub name: String,
    pub phone: Option<String>,
    pub address_id: i32,
}


#[get("/customer/<customer_id>")]
pub fn get_customer_wrapper(customer_id: i32) -> String {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let customer = get_customer(&mut conn, customer_id);

    match customer {
        Some(customer) => format!("{:?}", customer),
        None => "Customer not found".to_string(),
    }
}

fn get_customer(conn: &mut PgConnection, customer_id: i32) -> Option<Customer> {
    customer::table
        .find(customer_id)
        .first::<Customer>(conn)
        .optional()
        .unwrap()
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
