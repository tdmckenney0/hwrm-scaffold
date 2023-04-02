pub mod weapon;
pub mod weapon_result;
pub mod weapon_penetration;
pub mod weapon_accuracy;
pub mod weapon_angles;
pub mod weapon_misc;
pub mod weapon_turret_sound;

use std::path::Path;
use std::io::prelude::*;
use std::fs;
use std::fmt;
use std::collections::HashMap;

use diesel::prelude::*;

use weapon_result::{ WeaponResultCollection, NewWeaponResult, NewWeaponResultCollection };
use weapon_penetration::{ WeaponPenetrationCollection, NewWeaponPenetration, NewWeaponPenetrationCollection };
use weapon_accuracy::{ WeaponAccuracyCollection, NewWeaponAccuracy, NewWeaponAccuracyCollection };
use weapon_angles::{ WeaponAngles, NewWeaponAngles };
use weapon_misc::{ WeaponMisc, NewWeaponMisc };
use weapon_turret_sound::{ WeaponTurretSound, NewWeaponTurretSound };
use weapon::{ Weapon, WeaponCollection };

///
/// Weapon File (*.wepn)
///
#[derive(Debug)]
pub struct WeaponFile {
    pub weapon: Weapon,
    pub weapon_results: WeaponResultCollection,
    pub weapon_penetration: WeaponPenetrationCollection,
    pub weapon_accuracy: WeaponAccuracyCollection,
    pub weapon_angles: Option<WeaponAngles>,
    pub weapon_misc: Option<WeaponMisc>,
    pub weapon_turret_sound: Option<WeaponTurretSound>
}

impl fmt::Display for WeaponFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut wepn_file: Vec<String> = Vec::new();

        wepn_file.push(self.weapon.to_string());
        wepn_file.push(self.weapon_results.to_string());
        wepn_file.push(self.weapon_penetration.to_string());
        wepn_file.push(self.weapon_accuracy.to_string());

        if let Some(angles) = &self.weapon_angles {
            wepn_file.push(angles.to_string());
        }

        if let Some(misc) = &self.weapon_misc {
            wepn_file.push(misc.to_string());
        }

        if let Some(sound) = &self.weapon_turret_sound {
            wepn_file.push(sound.to_string());
        }

        write!(f, "{}", wepn_file.join("\r\n\r\n"))
    }
}

impl WeaponFile {
    // Get the Weapon File for a specific weapon name. Possible that it can't be found.
    pub fn get_for_weapon(connection: &mut SqliteConnection, weapon_name: &String) -> Option<Self> {
        let some_weapon: Option<Weapon> = Weapon::get_for_weapon(connection, weapon_name);

        if let Some(weapon) = some_weapon {
            let weapon_results = WeaponResultCollection::get_for_weapon(connection, weapon_name);
            let mut weapon_penetration = WeaponPenetrationCollection::get_for_weapon(connection, weapon_name);
            let mut weapon_accuracy = WeaponAccuracyCollection::get_for_weapon(connection, weapon_name);
            let weapon_angles = WeaponAngles::get_for_weapon(connection, weapon_name);
            let weapon_misc = WeaponMisc::get_for_weapon(connection, weapon_name);
            let weapon_turret_sound = WeaponTurretSound::get_for_weapon(connection, weapon_name);

            // Copy default values from `Weapon` model into collections for ease of exporting.
            weapon_accuracy.use_default_accuracy(&weapon);
            weapon_penetration.use_field_penetration(&weapon);
            weapon_penetration.use_default_penetration(&weapon);

            Some(Self {
                weapon,
                weapon_results,
                weapon_penetration,
                weapon_accuracy,
                weapon_angles,
                weapon_misc,
                weapon_turret_sound
            })
        } else {
            None
        }
    }

    /// Write WeaponFile to disk.
    pub fn write_to_disk(&self, path: &Path) -> Result<(), &str> {
        if !path.is_dir() {
            if let Ok(mut file) = fs::File::create(path) {
                let res = file.write_all(self.to_string().as_bytes());

                if res.is_err() {
                    Err("Could not write to file!")
                } else {
                    Ok(())
                }
            } else {
                Err("Could not open file!")
            }
        } else {
            Err("Cannot write *.wepn file to a directory!")
        }
    }

    /// Write WeaponFile out to a directory.
    pub fn write_to_dir(&self, dir: &Path) -> Result<(), &str> {
        if dir.is_dir() {
            let wepn_file = dir.join(format!("{}.wepn", self.weapon.name));

            self.write_to_disk(wepn_file.as_path())
        } else {
            Err("Could write weapon file to directory!")
        }
    }

    /// Write out the weapon file to the "weapon" directory. This creates the containment directory for the weapon as well.
    pub fn write_to_weapon_dir(&self, weapon_dir: &Path) -> Result<(), &str> {
        if weapon_dir.is_dir() {
            let containment_dir = weapon_dir.join(self.weapon.name.as_str());

            if !containment_dir.exists() {
                fs::create_dir(&containment_dir).expect("Could not create weapon containment directory!");
            }

            self.write_to_dir(containment_dir.as_path())
        } else {
            Err("Path is not a directory!")
        }
    }
}

pub struct WeaponFileCollection {
    pub weapon_files: HashMap<String, WeaponFile>
}

impl WeaponFileCollection {
    /// Create WeaponFileCollection from a WeaponCollection
    pub fn from_weapon_collection(connection: &mut SqliteConnection, mut weapons: WeaponCollection) -> Self {
        let weapon_names = weapons.get_names();
        let mut weapon_files = HashMap::new();

        let mut all_weapon_results = WeaponResultCollection::get_for_weapons(connection, &weapon_names).key_by_weapon_name();
        let mut all_weapon_penetrations = WeaponPenetrationCollection::get_for_weapons(connection, &weapon_names).key_by_weapon_name();
        let mut all_weapon_accuracies = WeaponAccuracyCollection::get_for_weapons(connection, &weapon_names).key_by_weapon_name();

        for (name, w) in weapons.weapons.drain() {
            let weapon_penetration = all_weapon_penetrations.remove(&name).unwrap_or(WeaponPenetrationCollection::new());
            let weapon_results = all_weapon_results.remove(&name).unwrap_or(WeaponResultCollection::new());
            let weapon_accuracy = all_weapon_accuracies.remove(&name).unwrap_or(WeaponAccuracyCollection::new());

            weapon_files.insert(name.to_string(), WeaponFile {
                weapon: w,
                weapon_results,
                weapon_penetration,
                weapon_accuracy,
                weapon_angles: None,
                weapon_misc: None,
                weapon_turret_sound: None
            });
        }

        Self {
            weapon_files
        }
    }

    /// Get all Weapon Files from the Database.
    pub fn get_all_weapon_files(connection: &mut SqliteConnection) -> Self {
        let weapons = WeaponCollection::get_all_weapons(connection);

        Self::from_weapon_collection(connection, weapons)
    }

    /// Get weapon files from a list of weapon_name Strings
    pub fn get_weapon_files_from_names(connection: &mut SqliteConnection, weapon_names: Vec<String>) -> Self {
        let weapons = WeaponCollection::get_weapons_from_names(connection, weapon_names);

        Self::from_weapon_collection(connection, weapons)
    }
}

///
/// NewWeaponFile
///
#[derive(Debug)]
pub struct NewWeaponFile {
    pub weapon: Weapon,
    pub weapon_results: NewWeaponResultCollection,
    pub weapon_penetration: NewWeaponPenetrationCollection,
    pub weapon_accuracy: NewWeaponAccuracyCollection,
    pub weapon_angles: Option<NewWeaponAngles>,
    pub weapon_misc: Option<NewWeaponMisc>,
    pub weapon_turret_sound: Option<NewWeaponTurretSound>
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
        let weapon_angles = NewWeaponAngles::from_string(&weapon_name, &contents);
        let weapon_misc = NewWeaponMisc::from_string(&weapon_name, &contents);
        let weapon_turret_sound = NewWeaponTurretSound::from_string(&weapon_name, &contents);

        let mut new_weapon_file = Self {
            weapon,
            weapon_results,
            weapon_penetration,
            weapon_accuracy,
            weapon_angles,
            weapon_misc,
            weapon_turret_sound
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
        use crate::schema::weapon_angles::dsl::*;
        use crate::schema::weapon_misc::dsl::*;
        use crate::schema::weapon_turret_sound::dsl::*;

        let mut weapons_to_insert: Vec<&Weapon> = Vec::new();
        let mut weapon_results_to_insert: Vec<&NewWeaponResult> = Vec::new();
        let mut weapon_penetrations_to_insert: Vec<&NewWeaponPenetration> = Vec::new();
        let mut weapon_accuracies_to_insert: Vec<&NewWeaponAccuracy> = Vec::new();
        let mut weapon_angles_to_insert: Vec<&NewWeaponAngles> = Vec::new();
        let mut weapon_misc_to_insert: Vec<&NewWeaponMisc> = Vec::new();
        let mut weapon_turret_sounds_to_insert: Vec<&NewWeaponTurretSound> = Vec::new();

        for weapon_file in self.new_weapon_files.iter() {
            weapons_to_insert.push(&weapon_file.weapon);

            if let Some(angles) = &weapon_file.weapon_angles {
                weapon_angles_to_insert.push(angles);
            }

            if let Some(sound) = &weapon_file.weapon_turret_sound {
                weapon_turret_sounds_to_insert.push(sound);
            }

            if let Some(wm) = &weapon_file.weapon_misc {
                weapon_misc_to_insert.push(wm);
            }

            weapon_file.weapon_results.weapon_results.iter().for_each(|x| weapon_results_to_insert.push(x));
            weapon_file.weapon_penetration.weapon_penetrations.iter().for_each(|x| weapon_penetrations_to_insert.push(x));
            weapon_file.weapon_accuracy.weapon_accuracies.iter().for_each(|x| weapon_accuracies_to_insert.push(x));
        }

        // Delete all weapons first.
        diesel::delete(weapon_turret_sound)
            .execute(connection)
            .expect("Error clearing weapon_turret_sound!");

        diesel::delete(weapon_misc)
            .execute(connection)
            .expect("Error clearing weapon_misc!");

        diesel::delete(weapon_accuracy)
            .execute(connection)
            .expect("Error clearing weapon_accuracy!");

        diesel::delete(weapon_penetrations)
            .execute(connection)
            .expect("Error clearing weapon_penetrations!");

        diesel::delete(weapon_results)
            .execute(connection)
            .expect("Error clearing weapon_results!");

        diesel::delete(weapon_angles)
            .execute(connection)
            .expect("Error clearing weapon_angles!");

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

        diesel::insert_into(weapon_angles)
            .values(weapon_angles_to_insert)
            .execute(connection)
            .expect("Could not insert weapon_angles!");

        diesel::insert_into(weapon_misc)
            .values(weapon_misc_to_insert)
            .execute(connection)
            .expect("Could not insert weapon_misc!");

        diesel::insert_into(weapon_turret_sound)
            .values(weapon_turret_sounds_to_insert)
            .execute(connection)
            .expect("Could not insert weapon_turret_sounds!");
    }
}
