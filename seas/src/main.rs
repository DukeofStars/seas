#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;

use jellyfish::jelly_rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/jellyfish/dist/", FileServer::from("dist"))
        .mount("/jellyfish/", routes![jelly_rocket::get_package])
}
