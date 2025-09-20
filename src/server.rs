#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
#[get("/")]
fn index() -> &'static str {
    "This is the homepage."
}
