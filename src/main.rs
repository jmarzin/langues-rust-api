#![feature(proc_macro_hygiene, decl_macro)]
#![feature(const_string_new)]

#[macro_use] extern crate rocket;

extern crate serde_json;
extern crate serde_derive;
extern crate rocket_cors;

#[macro_use] extern crate rocket_contrib;

use rocket_contrib::databases::postgres;

#[database("postgres_db")]
pub struct MyPgDatabase(postgres::Connection);

mod api;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {

    let cors = rocket_cors::CorsOptions {
        ..Default::default()
    }
        .to_cors()?;

    rocket::ignite().mount("/",
                           routes![api::api,
                                          api::v1_langues,
                                          api::v1_date_themes,
                                          api::v1_themes,
                                          api::v2_date_themes,
                                          api::v2_themes,
                                          api::v1_date_mots,
                                          api::v3_mots,
                                          api::v2_date_mots,
                                          api::v4_mots,
                                          api::v1_date_verbes,
                                          api::v1_verbes,
                                          api::v2_date_verbes,
                                          api::v2_verbes,
                                          api::v1_date_formes,
                                          api::v1_formes,
                                          api::v2_date_formes,
                                          api::v2_formes,
                                          api::v1_date_formestypes,
                                          api::v1_formestypes,
                                          api::v1_date_maj])
        .attach(MyPgDatabase::fairing())
        .attach(cors)
        .launch();
    Ok(())
}
