#[macro_use] extern crate rocket;

#[get("/")] // Default endpoint
fn index() -> &'static str {
    "More testing"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}