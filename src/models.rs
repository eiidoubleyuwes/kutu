//Creating a simple struct so the DB can be queried in Rust I dont want to keep on writing SQL queries.
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::watu)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct watu{
    pub id: i32,
    pub email: String,
    pub funguo: String,
    pub vault: String,
}