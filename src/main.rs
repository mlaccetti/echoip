#[macro_use] extern crate rocket;

mod handler;
mod model;
mod util;

use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![handler::index])
        .attach(Template::fairing())
}
