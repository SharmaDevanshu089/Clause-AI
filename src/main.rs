mod env;
mod server;
use google_generative_ai_rs::v1beta::VertexAI;

#[macro_use]
extern crate rocket;

const DEBUG: bool = true;
const LOCALE: &str = "asia-south1";

#[launch]
fn rocket() -> _ {
    env::conform_env();
    server::setup_server()
}
fn get_ai_online() {
    let api_key = env::get_api_key();
    let project_id = env::get_id();
    let location = LOCALE;
    let client = VertexAI::new(&project_id, &location, &api_key);
}
