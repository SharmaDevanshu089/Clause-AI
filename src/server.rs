pub use rocket::fs::FileServer;

#[post("/analyze")]
pub fn analyze() -> &'static str {
    "This is the AI response endpoint."
}
pub fn setup_server() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/", routes![analyze])
        .mount("/", FileServer::from("static"))
}
