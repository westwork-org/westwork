extern crate grpc;
extern crate protobuf;
extern crate futures;
extern crate futures_cpupool;
extern crate config;

use std::thread;
use std::sync::{Arc, Mutex};
use std::path::Path;

use config::{Config, File, FileFormat};

use grpc::result::GrpcResult;
use grpc::futures_grpc::*;

mod bootstrap;
mod bootstrap_grpc;

use bootstrap::*;
use bootstrap_grpc::*;

const settings_path: &str = "/data/conf/westwork/settings.yaml";

struct BootstrapImpl {
  config: Arc<Mutex<Config>>
}

impl BootstrapAsync for BootstrapImpl {
    fn SetName(&self, name: Name) -> GrpcFutureSend<Empty> {
        let cloned_conf = self.config.clone();
        let mut locked_conf = cloned_conf.lock().expect("Failed to lock config.");
        locked_conf.set(&"first_name", name.first);
        locked_conf.set(&"last_name", name.last);
        Box::new(futures::finished(Empty{}))       
    }

    fn SetUserName(&self, username: UserName) -> GrpcFutureSend<Empty> {
        let cloned_conf = self.config.clone();
        let mut locked_conf = cloned_conf.lock().expect("Failed to lock config.");
        locked_conf.set(&"username", username.username);
        Box::new(futures::finished(Empty{}))       
    }

    fn GetWifi(&self, _: Empty) -> GrpcFutureSend<WifiList> {

    }
}

fn main () {
    let bootstrap = BootstrapImpl{ Arc::new(Mutex::new(config::Config::new())) };
    let cloned_conf = bootstrap.config.clone();
    let mut this_conf = cloned_conf.lock().expect("Failed to lock config.");
    if Path::new(settings_path).exists() {
        this_conf.merge(File::new(settings_path, FileFormat::Yaml)).unwrap();
    } else {
        this_conf.set(&"configured", false);
    }
    drop(this_conf);

    let _server = BootstrapAsyncServer::new("[::]:50051", Default::default(), bootstrap);

    loop {
        thread::park();
    }
}
