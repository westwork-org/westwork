struct ClientConnection {
    authed_used: Option<User>,
}

impl ClientConnection {
    pub fn new (handle: websocket::client::Client) {
        // Loop to listen for messages
        // We will probably have to also spawn a second thread to send eventually
        let (mut sender, mut receiver) = client.split();

        // TODO: Make a channel here

        thread::spawn(move || {
            for message in receiver.incoming_messages() {
                let message: Message = message.unwrap();
                // Types of messages:
                // 1. Hello (first or reestablished connection) w/ auth token
                // 2. Messages
                // 3. Calendar
                // 4. Files(?)
                // 5. Logout
                match message.opcode() {
                    Type::Binary => {
                        println!("Got a binary websocket message??");
                        continue;
                    }
                    Type::Pong => {
                        // Right now I don't think we have to do anything with this.
                    }
                    Type::Ping => {
                        send_loop.send(message.into_pong().unwrap()); // This shouldn't ever fail.
                    }
                    Type::Close => {
                        println!("Received close from client, shutting down thread.");
                        send_loop.send("SHUTDOWN");
                        receiver.shutdown_receiver();
                        break;
                    }
                    Type::Text => {
                        
                    }


                }
            }
        })
        loop {
            // Sender loop here.
        }
    }
}