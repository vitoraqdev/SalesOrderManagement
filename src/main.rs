#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

pub mod schema;
pub mod libs;
mod mount;

pub(crate) static DATABASE_URL: &str = "postgres://postgres:postgres@localhost:5432/salesordermanagement";

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = mount::rocket()
        .launch()
        .await?;

    Ok(())
}