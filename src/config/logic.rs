#[post("/config"), data="<name>"]
fn get_name (name: Form<Name>) -> String {
    
}