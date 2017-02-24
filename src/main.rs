extern crate mustache;
extern crate rustc_serialize;

use std::io;
use std::path::Path;

#[derive(RustcEncodable)]
struct Instance {
    domain: String,
    admin_user: String,
    admin_name: String,
}

fn main () {
    let mut input = String::new();
    println!("Input your name, username, and domain separated by commas.");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(error) => println!("Error: {}", error)
    }
    let split: Vec<&str> = input.split(',').collect();
    let this_instance = Instance {
        domain: split[2].trim().to_string(),
        admin_user: split[1].to_string(),
        admin_name: split[0].to_string()
    };

    let compose_template = match mustache::compile_path(Path::new("docker-compose.yml.mustache")) {
        Ok(template) => template,
        Err(error) => panic!("Could not find docker-compose template: {}", error),
    };
    compose_template.render(&mut io::stdout(), &this_instance);
}