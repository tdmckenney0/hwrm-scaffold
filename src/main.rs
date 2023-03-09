pub mod schema;
pub mod models;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    use self::schema::weapons::dsl::*;

    let connection = &mut establish_connection();
    let results = weapons
        .limit(5)
        .load::<models::weapon::Weapon>(connection)
        .expect("Error loading weapons");

    println!("Displaying {} weapons", results.len());
    for weapon in results {
        println!("{}", weapon.name);
        println!("----------\n");
        println!("{}", weapon.weapon_type);
    }
}
