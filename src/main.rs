extern crate mustache;
extern crate rustc_serialize;
extern crate rand;

use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::path::Path;
use std::process::{Command, Stdio};
use rand::Rng;

#[derive(RustcEncodable)]
struct Instance {
    domain: String,
    admin_user: String,
    admin_name: String,
    root_pass: String,
}

fn main () {
    // Generate a random password then hash it for inserting into ldap
    // TODO: We should only do this the first time we're started up and save the results
    let clear_passwd: String = rand::thread_rng().gen_ascii_chars().take(20).collect();
    let output = match Command::new("mkpasswd").arg("-m").arg("SHA-512").arg("-s").stdin(Stdio::piped()).stdout(Stdio::piped()).spawn() {
        Ok(process) => process,
        Err(error) => panic!("Unable to generate password, do you have mkpasswd installed? {}", error),
    };
    match output.stdin.unwrap().write_all((clear_passwd + "\n").as_bytes()) {
        Ok(_) => {},
        Err(error) => panic!("Could not write password to mkpasswd. {}", error),
    }
    let mut passwd = String::new();
    match output.stdout.unwrap().read_to_string(&mut passwd) {
        Ok(_) => {},
        Err(error) => panic!("couldn't read mkpasswd stdout: {}", error),
    }
    passwd = "{SHA512}".to_string() + &passwd;
    passwd = passwd.trim().to_string();

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
        admin_name: split[0].to_string(),
        root_pass: passwd,
    };

    let compose_template = match mustache::compile_path(Path::new("docker-compose.yml.mustache")) {
        Ok(template) => template,
        Err(error) => panic!("Could not find docker-compose template: {}", error),
    };
    let mut f = File::create("docker-compose.yml").expect("Could not open docker-compose.yml file. Check permissions?");
    compose_template.render(&mut f, &this_instance);
}
