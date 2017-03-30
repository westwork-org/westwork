use rocket::State;
use config::Config;
use rocket_contrib::{JSON, Value};
use std::collections::HashMap;
use WestworkConf;

// TODO: How to handle names that don't follow the First Last template
#[derive(Deserialize)]
struct Name {
    first: String,
    last: String
}

#[post("/add_name", format = "application/json", data = "<name>")]
fn add_name(name: JSON<Name>, conf: State<WestworkConf>) -> JSON<Value> {
    // Receive the new user's name and insert into the state
    let mut this_conf = conf.lock().expect("Failed to lock conf");
    this_conf.set(&"first_name", name.0.first);
    this_conf.set(&"last_name", name.0.last);
    let mut resp = HashMap::new();
    resp.insert("success", "true");
    resp.insert("next_html", "<span id='boot-ques'>What would you like your username to be?</span>\
                              <div id='boot-form'>\
                              <form action='add_username'>\
                                  <input type='text' placeholder='Username'>\
                              </form>\
                              </div>");
    JSON(json!(resp))
}
