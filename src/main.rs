
use std::fs::File;

fn main () {
    rocket::ignite().mount("/", routes![index, get_css]).launch();
}
