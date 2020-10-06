#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::serve::StaticFiles;

fn main() {
    rocket::ignite().mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/client/public"))).launch();
}