#[macro_use] 
extern crate rocket;
use diesel::prelude::*;
use rocket_sync_db_pools::database;

pub mod schema;
pub mod models;
#[macro_use] 
extern crate diesel;

#[database("kutu")]
struct MyDatabase(diesel::PgConnection);
#[get("/")] // Default endpoint
fn index() -> &'static str {
    "More testing"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}