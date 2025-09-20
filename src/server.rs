pub use rocket::fs::FileServer;

#[post("/analyze")]
pub fn analyze() -> &'static str {
    "This is the AI response endpoint."
}
pub fn setup_server() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        // Your API route is mounted first.
        .mount("/", routes![analyze])
        // The FileServer serves your frontend from the "static" folder.
        .mount("/", FileServer::from("static"))
}
