// i just moved the entire code to main.rs , it was headache transporting datas

// // pub use rocket::fs::FileServer;

// #[post("/analyze")]
// pub fn analyze() -> &'static str {
//     "This is the AI response endpoint."
// }
// pub fn setup_server() -> rocket::Rocket<rocket::Build> {
//     rocket::build()
//         .mount("/", routes![analyze])
//         .mount("/", FileServer::from("static"))
// }

// mod env;
// use google_generative_ai_rs::v1::api::Client;
// use google_generative_ai_rs::v1::gemini::{Content, Part};
// use rocket::fs::FileServer;
// #[macro_use]
// extern crate rocket;

// const DEBUG: bool = true;
// const LOCALE: &str = "asia-south1";

// #[launch]
// fn rocket() -> _ {
//     env::conform_env();
//     setup_server()
// }
// fn get_ai_online() {
//     let api_key = env::get_api_key();
//     let project_id = env::get_id();
//     let location = LOCALE;
//     let client = Client::new(api_key);
// }
// #[post("/analyze")]
// pub fn analyze() -> &'static str {
//     "This is the AI response endpoint."
// }
// pub fn setup_server() -> rocket::Rocket<rocket::Build> {
//     rocket::build()
//         .mount("/", routes![analyze])
//         .mount("/", FileServer::from("static"))
// }
