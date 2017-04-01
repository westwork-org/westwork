use rocket::State;
use config::Config;
use rocket_contrib::{JSON, Value};
use std::collections::HashMap;
use WestworkConf;
use libc::c_int;
use std::ffi::CString;
use std::ptr;

// TODO: How to handle names that don't follow the First Last template
#[derive(Deserialize)]
struct Name {
    first: String,
    last: String
}
static IW_AUTH_WPA_VERSION_DISABLED: u8 =   0x00000001
static IW_AUTH_WPA_VERSION_WPA: u8 =		0x00000002
static IW_AUTH_WPA_VERSION_WPA2: u8 =     	0x00000004

#[derive(Deserialize)]
struct WifiNetwork{
    SSID: String,
    encryption: String,
    strength: i16,
}

enum iwrange {}
enum wireless_scan_head {}
enum wireless_scan {}

#[link(name="iwlib")]
extern {
    fn iw_socket_open() -> c_int;
    fn iw_get_range_info(socket: c_int,
                         interface: CString,
                         range: &iwrange) -> c_int;
    fn iw_scan(socket: c_int,
               interface: CString,
               version: c_int,
               head: &wireless_scan_head) -> c_int;
}

#[get("/wifi_list")]
fn wifi_list() -> JSON<Value> {
    // In order to join the user's local wireless network, we have to give them a list of available networks.
    // There's no Rust library that does this to date, so we have to call out to the iw library.
    // It is probably worth pulling this out into its own mod eventually, or perhaps its own crate.
    unsafe {
        // First get an iw socket.
        let sock = iw_socket_open();
        let interface_name = CString::new("wlan0").unwrap();
        let range: iwrange;
        let head: wireless_scan_head;
        if iw_get_range_info(sock, interface_name, &range) < 0 {
            // We have to make this call in order to get the version of the library on the computer
            let mut resp = HashMap::new();
            resp.insert("success", "false");
            resp.insert("error", "Could not retrieve wireless network list.");
            JSON(json!(resp))
        }
        if iw_scan(sock, interface_name, range.we_version_compiled, &head) <0 {
            // This is the actual scan call that fills in the `head` struct with information about the visible networks.
            let mut resp = HashMap::new();
            resp.insert("success", "false");
            resp.insert("error", "Could not retrieve wireless network list.");
            JSON(json!(resp))
        }
        result = head.result;
        let mut list = Vec::new();
        while result != ptr::null {
            // The scan results are a linked list of structs with a bunch of information about each network
            // The type of encryption is encoded in a bitflag called `key_flags` which we check by doing
            // a bitwise and against the known bitflags.
            let answer = if result.b.key_flags & IW_AUTH_WPA_VERSION_DISABLED > 0 {
                "None".to_string()
            } else if result.b.key_flags & IW_AUTH_WPA_VERSION_WPA > 0 {
                "WPA".to_string()
            } else if result.b.key_flags & IW_AUTH_WPA_VERSION_WPA2 > 0 {
                "WPA2".to_string()
            } ;
            list.push(WifiNetwork {
                SSID: result.b.essid.to_string(),
                encryption: answer, // TODO Figure out how to get encryption type from `result`
                strength: result.stats
            });
            result = result.next;
        }
        resp.insert("success", "false");
        resp.insert("networks", json!(list));
        JSON(json!(resp))

    }
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
