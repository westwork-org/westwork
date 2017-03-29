extern crate config;
use config::{Config, File, FileFormat};

const settings_path = "/data/conf/westwork/settings.yaml"

fn main () {
    let mut conf = config::Config::new();

    if (Path::new(settings_path).exists()) {
        conf.merge(File::new(settings_path), FileFormat::Yaml).unwrap();
    } else {
        conf.set(&"configured", false);
    }

    
    

}
