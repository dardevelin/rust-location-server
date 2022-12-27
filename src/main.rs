#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;


#[get("/")]
fn index() -> &'static str {
    "Your Location Server is Running!"
}

#[post("/location", data = "<location>")]
fn location(location: String) -> &'static str {
    println!("Location: {}", location);
    "Location Received"
}


fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
