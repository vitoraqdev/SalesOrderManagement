use super::super::schema::neighborhood;
use diesel::prelude::*;
use rocket::form::{Form, FromForm};
use rocket::http::Status;
use crate::DATABASE_URL;
use serde::Serialize;
use rocket::serde::json::Json;
use diesel::result::Error;

#[derive(Debug, Queryable, Serialize)]
pub struct Neighborhood {
    pub id: i32,
    pub name: String,
    pub delivery_fee: f64,
}


#[derive(Debug, AsChangeset, Insertable, FromForm)]
#[diesel(table_name = neighborhood)]
pub struct NewNeighborhood {
    pub name: String,
    pub delivery_fee: f64,
}


#[get("/address/neighborhood/<neighborhood_id>")]
pub fn get_neighborhood(neighborhood_id: i32) -> String {
    let mut conn = PgConnection::establish(&DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let neighborhood = _get_neighborhood(&mut conn, neighborhood_id);

    match neighborhood {
        Some(neighborhood) => format!("{:?}", neighborhood),
        None => format!("Neighborhood not found"),
    }
}

pub fn _get_neighborhood(conn: &mut PgConnection, neighborhood_id: i32) -> Option<Neighborhood> {
    neighborhood::table
        .filter(neighborhood::id.eq(neighborhood_id))
        .first::<Neighborhood>(conn)
        .optional()
        .unwrap()
}

#[post("/address/neighborhood", data = "<neighborhood>", format = "application/x-www-form-urlencoded")]
pub fn create_neighborhood(neighborhood: Form<NewNeighborhood>) -> String {
    let mut conn = PgConnection::establish(&DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));
    let new_neighborhood = diesel::insert_into(neighborhood::table)
        .values(neighborhood.into_inner())
        .get_result::<Neighborhood>(&mut conn)
        .expect("Error creating neighborhood");
    format!("{:?}", new_neighborhood)
}

#[get("/address/neighborhood")]
pub fn get_neighborhoods() -> Json<Vec<Neighborhood>> {
    let mut conn = PgConnection::establish(&DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let neighborhoods = _get_neighborhoods(&mut conn);

    Json(neighborhoods)
}

pub fn _get_neighborhoods(conn: &mut PgConnection) -> Vec<Neighborhood> {
    neighborhood::table
        .load::<Neighborhood>(conn)
        .expect("Error loading neighborhoods")
}

#[put("/address/neighborhood/<neighborhood_id>", data = "<neighborhood>")]
pub fn update_neighborhood(neighborhood_id: i32, neighborhood: Form<NewNeighborhood>) -> Result<Json<Neighborhood>, (Status, String)> {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let updated_neighborhood = _update_neighborhood(&mut conn, neighborhood_id, neighborhood.into_inner());

    match updated_neighborhood {
        Ok(updated_neighborhood) => Ok(Json(updated_neighborhood)),
        Err(err) => match err {
            Error::NotFound => Err((Status::NotFound, "Neighborhood not found".to_string())),
            Error::DatabaseError(_, info) => Err((Status::InternalServerError, info.message().to_string())),
            _ => Err((Status::InternalServerError, "Internal Server Error".to_string())),
        }
    }
}

pub fn _update_neighborhood(conn: &mut PgConnection, neighborhood_id: i32, neighborhood: NewNeighborhood) -> QueryResult<Neighborhood> {
    diesel::update(neighborhood::table)
        .filter(neighborhood::id.eq(neighborhood_id))
        .set(neighborhood)
        .get_result::<Neighborhood>(conn)
}

#[delete("/address/neighborhood/<neighborhood_id>")]
pub fn delete_neighborhood(neighborhood_id: i32) -> Result<Status, (Status, String)> {
    let mut conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let deleted_neighborhood = _delete_neighborhood(&mut conn, neighborhood_id);

    match deleted_neighborhood {
        Ok(_) => Ok(Status::Ok),
        Err(err) => match err {
            Error::NotFound => Err((Status::NotFound, "Neighborhood not found".to_string())),
            Error::DatabaseError(_, info) => Err((Status::InternalServerError, info.message().to_string())),
            _ => Err((Status::InternalServerError, "Internal Server Error".to_string())),
        }
    }
}

pub fn _delete_neighborhood(conn: &mut PgConnection, neighborhood_id: i32) -> QueryResult<usize> {
    diesel::delete(neighborhood::table)
        .filter(neighborhood::id.eq(neighborhood_id))
        .execute(conn)
}
