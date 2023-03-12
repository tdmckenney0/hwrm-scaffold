pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::insert_into;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    use crate::schema::weapons::dsl::*;

    let connection = &mut establish_connection();
    let results = weapons
        .limit(5)
        .load::<models::weapon::Weapon>(connection)
        .expect("Error loading weapons");

    println!("Displaying {} weapons", results.len());
    for weapon in &results {
        println!("{}", weapon.name);
        println!("----------\n");
        println!("{}", weapon.weapon_type);
    }

    let results_wr = models::weapon::WeaponResult::belonging_to(&results)
        .select(models::weapon::WeaponResult::as_select())
        .load(connection)
        .expect("Error loading weapon results");

    println!("Displaying {} weapon results", results_wr.len());
    for weapon_result in results_wr {
        let formatted_spawn_weapon_id = match weapon_result.spawn_weapon_id {
            Some(n) => n.to_string(),
            None => "".to_string(),
        };

        println!("AddWeaponResult(NewWeaponType, \"{}\", \"{}\", \"{}\", {}, {}, \"{}\");",
            weapon_result.condition,
            weapon_result.effect,
            weapon_result.target,
            weapon_result.minimum_effect,
            weapon_result.maximum_effect,
            formatted_spawn_weapon_id
        );
    }

    let start_weapon_config = models::weapon::StartWeaponConfig::new("hgn_dummyfrigategun2".to_string());

    diesel::delete(weapons.filter(name.like(&start_weapon_config.name)))
        .execute(connection)
        .expect("Error deleting posts");

    insert_into(weapons)
        .values(start_weapon_config)
        .execute(connection)
        .expect("Error inserting to weapons");
}
