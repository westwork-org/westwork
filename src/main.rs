#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

use std::collections::HashMap;
use rocket_contrib::Template;

const FIRST_RUN: bool = true;

#[get("/")]
fn index () -> Template {
    /*
    If this is the first time being run, begin the initial configuration wizard
    Otherwise check the user's session.
        a) If logged in, serve the default Westwork package
        b) If not logged in, redirect to the login package
    */
    if FIRST_RUN {
        let context: HashMap<String, String> = HashMap::new(); // Its ok to just create an empty context
        Template::render("bootstrap", &context)
    } else {
        let mut context = HashMap::new();
        context.insert("firstname", "Ross");
        context.insert("lastname", "Schulman");
        Template::render("index", &context)
    }
}

fn main () {
    rocket::ignite().mount("/", routes![index]).launch();
}
