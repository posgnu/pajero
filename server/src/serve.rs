#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn serve() {
    rocket::ignite().mount("/", routes![index]).launch();
}
