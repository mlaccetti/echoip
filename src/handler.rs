use rocket::Request;
use rocket::response::Redirect;

use rocket_dyn_templates::{Template, handlebars, context};

use self::handlebars::{Handlebars, JsonRender};
use handlebars::{Renderable};

#[get("/")]
pub fn index() -> Template {
    Template::render("index", !context {
        title: "Hello",
        name: "Blah",
        items: vec!["One", "Two", "Three"],
    })
}