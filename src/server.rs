use rocket::fs::FileServer;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
#[get("/")]
fn index() -> &'static str {
    "This is the homepage."
}
#[post("/analyze")]
fn analyze() -> &'static str {
    //will add the AI logic here later
    "This is the AI response endpoint."
}
