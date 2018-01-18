#![feature(plugin, custom_derive, const_fn, decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use dotenv::dotenv;
use std::env;
use routes::*;

mod schema;
mod models;
mod db;
mod static_files;
mod routes;

fn rocket() -> rocket::Rocket {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");

    let pool = db::init_pool(database_url);
    rocket::ignite()
        .manage(pool)
        .mount(
            "/api/v1/",
            routes![index, new, show, delete, author, update],
        )
        .mount("/", routes![static_files::all, static_files::index])
        .catch(catchers![not_found])
}

fn main() {
    rocket().launch();
}
