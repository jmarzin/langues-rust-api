#![feature(proc_macro_hygiene, decl_macro)]
#![feature(const_string_new)]

#[macro_use] extern crate rocket;

extern crate serde_json;

extern crate rocket_contrib;

mod langues;

fn main() {
    rocket::ignite().mount("/",
                           routes![api::langues])
        .attach(cors)
        .launch();
    Ok(())
}
