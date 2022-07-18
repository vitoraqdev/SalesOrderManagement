use super::super::schema::neighborhood;
use diesel::prelude::*;
use rocket::form::{Form, FromForm};
use crate::DATABASE_URL;


#[derive(Debug, Queryable)]
pub struct Neighborhood {
    pub id: i32,
    pub name: String,
    pub delivery_fee: f64,
}


#[derive(Debug, Insertable, FromForm)]
#[table_name = "neighborhood"]
pub struct NewNeighborhood {
    pub name: String,
    pub delivery_fee: f64,
}


#[get("/address/neighborhood/<neighborhood_id>")]
pub fn get_neighborhood_wrapper(neighborhood_id: i32) -> String {
    let conn = PgConnection::establish(&DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let neighborhood = get_neighborhood(&conn, neighborhood_id);

    match neighborhood {
        Some(neighborhood) => format!("{:?}", neighborhood),
        None => format!("Neighborhood not found"),
    }
}

pub fn get_neighborhood(conn: &PgConnection, neighborhood_id: i32) -> Option<Neighborhood> {
    neighborhood::table
        .filter(neighborhood::id.eq(neighborhood_id))
        .first::<Neighborhood>(conn)
        .optional()
        .unwrap()
}

#[post("/address/neighborhood", data = "<neighborhood>", format = "application/x-www-form-urlencoded")]
pub fn create_neighborhood(neighborhood: Form<NewNeighborhood>) -> String {
    let conn = PgConnection::establish(&DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));
    let new_neighborhood = diesel::insert_into(neighborhood::table)
        .values(neighborhood.into_inner()   )
        .get_result::<Neighborhood>(&conn)
        .expect("Error creating neighborhood");
    format!("{:?}", new_neighborhood)
}