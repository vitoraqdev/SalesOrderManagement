use super::super::schema::motoboy;
use diesel::prelude::*;
use rocket::form::{Form, FromForm};
use crate::DATABASE_URL;


#[derive(Debug, Queryable)]
pub struct Motoboy {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub daily_salary: f64,
    pub is_active: bool,
}


#[derive(Debug, Insertable, FromForm)]
#[table_name="motoboy"]
pub struct NewMotoboy {
    pub name: String,
    pub phone: String,
    pub daily_salary: f64,
    pub is_active: bool,
}

#[get("/motoboy/<motoboy_id>")]
pub fn get_motoboy_wrapper(motoboy_id: i32) -> String {
    let conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let motoboy = get_motoboy(&conn, motoboy_id);

    match motoboy {
        Some(motoboy) => format!("{:?}", motoboy),
        None => "Motoboy not found".to_string(),
    }
}

pub fn get_motoboy(conn: &PgConnection, motoboy_id: i32) -> Option<Motoboy> {
    motoboy::table
        .find(motoboy_id)
        .first::<Motoboy>(conn)
        .optional()
        .unwrap()
}

#[post("/motoboy", data = "<motoboy>")]
pub fn create_motoboy(motoboy: Form<NewMotoboy>) -> String {
    let conn = PgConnection::establish(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL));

    let motoboy = diesel::insert_into(motoboy::table)
        .values(motoboy.into_inner())
        .get_result::<Motoboy>(&conn)
        .expect("Error creating motoboy");

    format!("{:?}", motoboy)
}