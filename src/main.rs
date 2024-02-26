#[macro_use] 
extern crate rocket;
use diesel::prelude::*;
mod schema;
mod models;
#[macro_use] 
extern crate diesel;
#[get("/")] // Default endpoint
fn index() -> &'static str {
    "More testing"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}