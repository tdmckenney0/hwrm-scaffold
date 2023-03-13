pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::insert_into;
use dotenvy::dotenv;
use std::env;
use std::fs;
use std::path::PathBuf;

fn find_files_with_extension(root_path: &PathBuf, extension: &str) -> Vec<PathBuf> {
    let mut files = vec![];
    if let Ok(entries) = fs::read_dir(root_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && path.extension().unwrap_or_default() == extension {
                    files.push(path);
                } else if path.is_dir() {
                    let mut sub_files = find_files_with_extension(&path, extension);
                    files.append(&mut sub_files);
                }
            }
        }
    }
    files
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    use crate::schema::weapons::dsl::*;

    let connection = &mut establish_connection();
    let root_path = PathBuf::from(env::var("WEAPON_DIR").expect("WEAPON_DIR must be set"));
    let extension = "wepn";
    let files = find_files_with_extension(&root_path, extension);
    let mut weapons_to_insert: Vec<models::weapon::StartWeaponConfig> = Vec::new();

    // Delete all weapons first.
    diesel::delete(weapons)
        .execute(connection)
        .expect("Error clearing weapons!");

    for file in files {
        let weapon_name = file.file_stem().unwrap().to_str().unwrap();

        let contents = fs::read_to_string(&file)
            .expect("Should have been able to read the file");

        let lines = contents.lines();

        for line in lines {
            if line.starts_with("StartWeaponConfig") {
                println!("parsing {}...", weapon_name);

                weapons_to_insert.push(models::weapon::StartWeaponConfig::new(weapon_name.to_string(), line.to_string()));
            }
        }
    }

    insert_into(weapons)
        .values(weapons_to_insert)
        .execute(connection);

    // List Weapons.
    let results = weapons
        .limit(10)
        .load::<models::weapon::Weapon>(connection)
        .expect("Error loading weapons");

    println!("Displaying {} weapons\n------------\n", results.len());

    for weapon in &results {
        println!("{} -> {}\n", weapon.name, weapon);
    }
}
