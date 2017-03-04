extern crate websocket;

use std::path::Path;
use std::thread;
use websocket::{Server, Message};

fn main () {

    let server = match Server::bind("127.0.0.1:8000") {
        Ok(connection) => connection,
        Err(e) => panic!("Could not bind websocket server: \{:?}", e)
    };

    for new_conn in server {
        thread::spawn(move || {
            let request = new_conn.unwrap().read_request().unwrap();
            let response = request.accept(); // Form a response
            let mut client = response.send().unwrap(); // Send the response
            ClientConnection::new(client);
        });
    }
}
