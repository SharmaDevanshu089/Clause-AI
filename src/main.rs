mod env;
mod server;

#[macro_use]
extern crate rocket;

const DEBUG: bool = true;
const LOCALE = "asia-south1";

#[launch]
fn rocket() -> _ {
    env::conform_env();
    server::setup_server()
}
