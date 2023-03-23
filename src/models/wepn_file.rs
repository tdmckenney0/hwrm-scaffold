pub mod weapon;
pub mod weapon_result;
pub mod weapon_penetration;
pub mod weapon_accuracy;

use std::path::Path;
use std::fs;

use diesel::prelude::*;

use weapon_result::{ WeaponResult, NewWeaponResult, NewWeaponResultCollection };
use weapon_penetration::{ NewWeaponPenetration, NewWeaponPenetrationCollection };
use weapon_accuracy::{ NewWeaponAccuracy, NewWeaponAccuracyCollection };
use weapon::Weapon;

///
/// Weapon File (*.wepn)
///
#[derive(Debug)]
pub struct WeaponFile {
    weapon: Weapon,
    weapon_results: Vec<WeaponResult>,
    weapon_penetration: NewWeaponPenetrationCollection,
    weapon_accuracy: NewWeaponAccuracyCollection
}

///
/// NewWeaponFile
///
#[derive(Debug)]
pub struct NewWeaponFile {
    weapon: Weapon,
    weapon_results: NewWeaponResultCollection,
    weapon_penetration: NewWeaponPenetrationCollection,
    weapon_accuracy: NewWeaponAccuracyCollection
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
        let weapon_penetration = NewWeaponPenetrationCollection::from_string(&weapon_name, &contents);
        let weapon_accuracy = NewWeaponAccuracyCollection::from_string(&weapon_name, &contents);

        let mut new_weapon_file = Self {
            weapon,
            weapon_results,
            weapon_penetration,
            weapon_accuracy
        };

        new_weapon_file.set_weapon_penetration();
        new_weapon_file.set_weapon_accuracy();

        new_weapon_file
    }

    /// Copies the Weapon Penetration from `weapon_penetration` to `weapon` for saving.
    pub fn set_weapon_penetration(&mut self) {
        self.weapon.field_penetration = self.weapon_penetration.field_penetration;
        self.weapon.default_penetration = self.weapon_penetration.default_penetration;
    }

    /// Copies the Weapon Accuracy from `weapon_accuracy` to `weapon` for saving.
    pub fn set_weapon_accuracy(&mut self) {
        self.weapon.default_accuracy = self.weapon_accuracy.default_accuracy;
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
        use crate::schema::weapon_penetrations::dsl::*;
        use crate::schema::weapon_accuracy::dsl::*;

        let mut weapons_to_insert: Vec<&Weapon> = Vec::new();
        let mut weapon_results_to_insert: Vec<&NewWeaponResult> = Vec::new();
        let mut weapon_penetrations_to_insert: Vec<&NewWeaponPenetration> = Vec::new();
        let mut weapon_accuracies_to_insert: Vec<&NewWeaponAccuracy> = Vec::new();

        for weapon_file in self.new_weapon_files.iter() {
            weapons_to_insert.push(&weapon_file.weapon);

            weapon_file.weapon_results.weapon_results.iter().for_each(|x| weapon_results_to_insert.push(x));
            weapon_file.weapon_penetration.weapon_penetrations.iter().for_each(|x| weapon_penetrations_to_insert.push(x));
            weapon_file.weapon_accuracy.weapon_accuracies.iter().for_each(|x| weapon_accuracies_to_insert.push(x));
        }

        // Delete all weapons first.
        diesel::delete(weapon_accuracy)
            .execute(connection)
            .expect("Error clearing weapon_accuracy!");

        diesel::delete(weapon_penetrations)
            .execute(connection)
            .expect("Error clearing weapon_penetrations!");

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

        diesel::insert_into(weapon_penetrations)
            .values(weapon_penetrations_to_insert)
            .execute(connection)
            .expect("Could not insert weapon_penetrations!");

        diesel::insert_into(weapon_accuracy)
            .values(weapon_accuracies_to_insert)
            .execute(connection)
            .expect("Could not insert weapon_accuracies!");
    }
}
