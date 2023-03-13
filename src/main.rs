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

    println!("Displaying {} weapons\n------------\n", results.len());
    for weapon in &results {
        println!("{} -> {}\n", weapon.name, weapon);
    }

    let results_wr = models::weapon::WeaponResult::belonging_to(&results)
        .select(models::weapon::WeaponResult::as_select())
        .load(connection)
        .expect("Error loading weapon results");

    println!("Displaying {} weapon results\n------------\n", results_wr.len());
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

    let weapon_config_lua = "StartWeaponConfig(NewWeaponType,\"AnimatedTurret\",\"Bullet\",\"kinetic_rapid\",\"Normal\",1200,5000,0,0,0,0,1,1,1,0.1,2,5,1,1,120,120,0.1,\"Normal\",0,0,0)".to_string();
    let start_weapon_config = models::weapon::StartWeaponConfig::new("hgn_dummyfrigategun3".to_string(), weapon_config_lua);

    diesel::delete(weapons.filter(name.like(&start_weapon_config.name)))
        .execute(connection)
        .expect("Error deleting `hgn_dummyfrigategun3`");

    insert_into(weapons)
        .values(start_weapon_config)
        .execute(connection)
        .expect("Error inserting to weapons");
}
