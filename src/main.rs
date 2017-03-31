#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate config;
extern crate rocket;
extern crate serde_json;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use config::{Config, File, FileFormat};
use rocket_contrib::{JSON, Value};
use rocket::State;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

mod bootstrap;

const settings_path: &str = "/data/conf/westwork/settings.yaml";

type WestworkConf = Arc<Mutex<Config>>;

fn main () {
    let conf = Arc::new(Mutex::new(config::Config::new()));
    let cloned_conf = conf.clone();
    let mut this_conf = cloned_conf.lock().expect("Failed to lock config.");
    if Path::new(settings_path).exists() {
        this_conf.merge(File::new(settings_path, FileFormat::Yaml)).unwrap();
    } else {
        this_conf.set(&"configured", false);
    }
    drop(this_conf);


    rocket::ignite().mount("/", routes![bootstrap::add_name]).manage(conf).launch();

}
