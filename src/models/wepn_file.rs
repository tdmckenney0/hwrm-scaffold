pub mod weapon;
pub mod weapon_result;

use std::path::Path;
use std::fs;

use diesel::prelude::*;

use weapon_result::{ WeaponResult, NewWeaponResult, NewWeaponResultCollection };
use weapon::Weapon;

///
/// Weapon File (*.wepn)
///
#[derive(Debug)]
pub struct WeaponFile {
    weapon: Weapon,
    weapon_results: Vec<WeaponResult>,
}

///
/// NewWeaponFile
///
#[derive(Debug)]
pub struct NewWeaponFile {
    weapon: Weapon,
    weapon_results: NewWeaponResultCollection,
}

impl NewWeaponFile {
    /// Create new `NewWeaponFile` from `Path`
    pub fn from_path(wepn_file: &Path) -> Self {
        let weapon_name = wepn_file.file_stem()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_string();

        let contents = fs::read_to_string(&wepn_file)
                                .expect("Should have been able to read the file");

        let weapon = Weapon::from_string(&weapon_name, &contents);
        let weapon_results = NewWeaponResultCollection::from_string(&weapon_name, &contents);

        Self {
            weapon,
            weapon_results
        }
    }
}

#[derive(Debug)]
pub struct NewWeaponFileCollection {
    pub new_weapon_files: Vec<NewWeaponFile>
}

impl NewWeaponFileCollection {
    /// Create from a Vector of NewWeaponFile
    pub fn from_vec(new_weapon_files: Vec<NewWeaponFile>) -> Self {
        Self {
            new_weapon_files
        }
    }

    /// Consumes the collection, inserts into database
    pub fn insert(self, connection: &mut SqliteConnection) {
        use crate::schema::weapons::dsl::*;
        use crate::schema::weapon_results::dsl::*;

        let mut weapons_to_insert: Vec<&Weapon> = Vec::new();
        let mut weapon_results_to_insert: Vec<&NewWeaponResult> = Vec::new();

        for weapon_file in self.new_weapon_files.iter() {
            weapons_to_insert.push(&weapon_file.weapon);

            weapon_file.weapon_results.weapon_results.iter().for_each(|x| weapon_results_to_insert.push(x));
        }

        // Delete all weapons first.
        diesel::delete(weapon_results)
            .execute(connection)
            .expect("Error clearing weapon_results!");

        diesel::delete(weapons)
            .execute(connection)
            .expect("Error clearing weapons!");

        // Insert our  weapons
        diesel::insert_into(weapons)
            .values(weapons_to_insert)
            .execute(connection)
            .expect("Could not insert weapons!");

        diesel::insert_into(weapon_results)
            .values(weapon_results_to_insert)
            .execute(connection)
            .expect("Could not insert weapon_results!");
    }
}
