use super::super::schema::motoboy;
use diesel::prelude::*;
use rocket::form::{Form, FromForm};
use crate::DATABASE_URL;
use serde::Serialize;
use rocket::serde::json::Json;
use rocket::response::status;


#[derive(Debug, Queryable, Serialize)]
pub struct Motoboy {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub daily_salary: f64,
    pub is_active: bool,
}


#[derive(Debug, AsChangeset, Insertable, FromForm)]
#[table_name="motoboy"]
pub struct NewMotoboy {
    pub name: String,
    pub phone: String,
    pub daily_salary: f64,
    pub is_active: bool,
}

#[get("/motoboy/<motoboy_id>")]
pub fn get_motoboy(motoboy_id: i32) -> Result<Json<Motoboy>, status::NotFound<String>> {
    let conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let motoboy = _get_motoboy(&conn, motoboy_id);

    match motoboy {
        Some(motoboy) => Ok(Json(motoboy)),
        None => Err(status::NotFound("Motoboy not found".to_string())),
    }
}

pub fn _get_motoboy(conn: &PgConnection, motoboy_id: i32) -> Option<Motoboy> {
    motoboy::table
        .find(motoboy_id)
        .first::<Motoboy>(conn)
        .optional()
        .unwrap()
}

#[post("/motoboy", data = "<motoboy>")]
pub fn create_motoboy(motoboy: Form<NewMotoboy>) -> Result<Json<Motoboy>, status::BadRequest<String>> {
    let conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let motoboy = _create_motoboy(&conn, motoboy.into_inner());

    match motoboy {
        Ok(motoboy) => Ok(Json(motoboy)),
        Err(_) => Err(status::BadRequest(Some("Error creating motoboy".to_string()))),
    }
}

fn _create_motoboy(conn: &PgConnection, motoboy: NewMotoboy) -> QueryResult<Motoboy> {
    diesel::insert_into(motoboy::table)
        .values(motoboy)
        .get_result::<Motoboy>(conn)
}

#[put("/motoboy/<motoboy_id>", data = "<motoboy>")]
pub fn update_motoboy(motoboy_id: i32, motoboy: Form<NewMotoboy>) -> Result<Json<Motoboy>, status::BadRequest<String>> {
    let conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let motoboy = _update_motoboy(&conn, motoboy_id, motoboy.into_inner());

    match motoboy {
        Ok(motoboy) => Ok(Json(motoboy)),
        Err(_) => Err(status::BadRequest(Some("Error updating motoboy".to_string()))),
    }
}

fn _update_motoboy(conn: &PgConnection, motoboy_id: i32, motoboy: NewMotoboy) -> QueryResult<Motoboy> {
    diesel::update(motoboy::table.find(motoboy_id))
        .set(motoboy)
        .get_result::<Motoboy>(conn)
}

#[delete("/motoboy/<motoboy_id>")]
pub fn delete_motoboy(motoboy_id: i32) -> Result<Json<Motoboy>, status::BadRequest<String>> {
    let conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let motoboy = _delete_motoboy(&conn, motoboy_id);

    match motoboy {
        Ok(motoboy) => Ok(Json(motoboy)),
        Err(_) => Err(status::BadRequest(Some("Error deleting motoboy".to_string()))),
    }
}

fn _delete_motoboy(conn: &PgConnection, motoboy_id: i32) -> QueryResult<Motoboy> {
    diesel::delete(motoboy::table.find(motoboy_id))
        .get_result::<Motoboy>(conn)
}

#[get("/motoboy")]
pub fn get_motoboys() -> Json<Vec<Motoboy>> {
    let conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let motoboys = _get_motoboys(&conn);

    Json(motoboys)
}

fn _get_motoboys(conn: &PgConnection) -> Vec<Motoboy> {
    motoboy::table
        .load::<Motoboy>(conn)
        .unwrap()
}
