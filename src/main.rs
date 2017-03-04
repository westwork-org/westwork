extern crate websocket;

use std::fs::File;
use std::thread;
use websocket::{Server, Message};

mod configure;

fn main () {
    let mut configured = true;
    let config_file = Path::new("/etc/westwork/westwork.conf");
    if config_file.exists() {
        // Read it into a struct somehow
    } else {
        configured = false;
    }

    let server = match Server::bind("127.0.0.1:8000") {
        Ok(connection) => connection,
        Err(e) => panic!("Could not bind websocket server: \{:?}", e)
    };

    for new_conn in server {
        thread::spawn(move || {
            let request = new_conn.unwrap().read_request().unwrap();
            let response = request.accept(); // Form a response
            let mut client = response.send().unwrap(); // Send the response

            if configured {
                ClientConnection::new(client);
            } else {
                configure::Configure::with_connection(client);
            }
            
        });
    }
}
