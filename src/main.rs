#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate config;
extern crate rocket;

use config::{Config, File, FileFormat};

mod bootstrap;

const settings_path: String = "/data/conf/westwork/settings.yaml"

fn main () {
    let mut conf = config::Config::new();

    if (Path::new(settings_path).exists()) {
        conf.merge(File::new(settings_path), FileFormat::Yaml).unwrap();
    } else {
        conf.set(&"configured", false);
    }


    rocket::ignite().mount("/", routes!([bootstrap::add_name])).manage(conf).launch();

}
