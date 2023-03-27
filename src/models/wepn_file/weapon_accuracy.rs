use regex::Regex;
use diesel::prelude::*;
use crate::schema::{weapon_accuracy};
use super::weapon::Weapon;
use std::fmt;
use std::collections::HashMap;

///
/// Regex's
///

/// Find the `setAccuracy(...)` function call in a string, parse into groups.
pub fn set_accuracy_regex() -> Regex {
    Regex::new(r"setAccuracy\(NewWeaponType\s*,\s*([0-9\.]*)\s*,\s*([\sA-za-z0-9=\.\{\},]*)\s*\)").unwrap()
}

/// Find the armor family arguments in `setAccuracy(...)`
pub fn armor_family_regex() -> Regex {
    Regex::new(r"\{\s*(\w*)\s*=\s*(\d*\.*\d*)\s*,\s*(?:damage\s*=\s*(\d*\.*\d*))*\s*,*\}").unwrap()
}

///
/// Weapon Accuracy
///
#[derive(Queryable, Selectable, Identifiable, Associations, Debug)]
#[diesel(belongs_to(Weapon, foreign_key = weapon_name))]
#[diesel(table_name = weapon_accuracy)]
pub struct WeaponAccuracy {
    pub id: i32,
    pub weapon_name: String,
    pub armor_family: String,
    pub accuracy: f32,
    pub damage: f32
}

impl fmt::Display for WeaponAccuracy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}={}, damage={},}}",
            self.armor_family,
            self.accuracy,
            self.damage
        )
    }
}

/// Collection of `WeaponAccuracy` with helper methods. Has additional arguments for the `setAccuracy` lua call.
#[derive(Debug)]
pub struct WeaponAccuracyCollection {
    pub default_accuracy: f32,
    pub weapon_accuracies: Vec<WeaponAccuracy>
}

impl WeaponAccuracyCollection {
    /// Change a Vector of WeaponAccuracy into a Collection.
    pub fn from_vec(default_accuracy: f32, weapon_accuracies: Vec<WeaponAccuracy>) -> Self {
        Self {
            default_accuracy,
            weapon_accuracies
        }
    }

    /// Get weapon accuracy for a specific weapon name. Possible that it can't be found.
    /// TODO: Handle `default_accuracy`
    pub fn get_for_weapon(connection: &mut SqliteConnection, name: &String) -> Self {
        use crate::schema::weapon_accuracy::dsl::*;

        let vec = weapon_accuracy
                    .filter(weapon_name.eq(name))
                    .load::<WeaponAccuracy>(connection)
                    .expect("Error loading weapon accuracy!");

        Self {
            default_accuracy: 0.0,
            weapon_accuracies: vec
        }
    }

    /// Get weapon accuracy for a list of weapon names.
    /// TODO: Handle `default_accuracy`
    pub fn get_for_weapons(connection: &mut SqliteConnection, names: &Vec<String>) -> Self {
        use crate::schema::weapon_accuracy::dsl::*;

        let vec = weapon_accuracy
                    .filter(weapon_name.eq_any(names))
                    .load::<WeaponAccuracy>(connection)
                    .expect("Error loading weapon accuracies!");

        Self {
            default_accuracy: 0.0,
            weapon_accuracies: vec
        }
    }

    /// Copy the `default_accuracy` value from weapon so it can be exported.
    pub fn use_default_accuracy(&mut self, weapon: &Weapon) {
        self.default_accuracy = weapon.default_accuracy;
    }

    /// Consume the collection, divide into HashMap by `weapon_name`
    pub fn key_by_weapon_name(self) -> HashMap<String, Self> {
        let mut weapon_accuracies: Vec<WeaponAccuracy> = self.weapon_accuracies;
        let mut map = HashMap::new();

        while let Some(wa) = weapon_accuracies.pop() {
            if !map.contains_key(&wa.weapon_name) {
                map.insert(wa.weapon_name.to_string(), Self {
                    default_accuracy: self.default_accuracy,
                    weapon_accuracies: vec![wa]
                });
            } else {
                map
                    .get_mut(&wa.weapon_name)
                    .expect("Could not find key??")
                    .weapon_accuracies.push(wa);
            }
        }

        map
    }
}

impl fmt::Display for WeaponAccuracyCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted = self.weapon_accuracies
                                .iter()
                                .map(|wa| wa.to_string())
                                .collect::<Vec<String>>()
                                .join(",");

        write!(f, "setAccuracy(NewWeaponType,{},{});",
            self.default_accuracy,
            formatted
        )
    }
}

///
/// NewWeaponAccuracy
///

/// Class Ready to be Inserted via Diesel
#[derive(Default, Insertable, Debug)]
#[diesel(table_name = weapon_accuracy)]
pub struct NewWeaponAccuracy {
    pub weapon_name: String,
    pub armor_family: String,
    pub accuracy: f32,
    pub damage: f32
}

/// Collection of `NewWeaponAccuracy` with helper methods. Has additional arguments from the `setAccuracy` lua call.
#[derive(Debug)]
pub struct NewWeaponAccuracyCollection {
    pub default_accuracy: f32,
    pub weapon_accuracies: Vec<NewWeaponAccuracy>
}

impl NewWeaponAccuracyCollection {
    /// Parse the `setAccuracy(...)` lua call into an instance.
    /// This whole process needs to be optimized at some point.
    pub fn from_string(weapon_name: &String, body: &String) -> Self {
        let rx = set_accuracy_regex();
        let rx_family = armor_family_regex();

        let caps = rx.captures(body);

        let mut default_accuracy: f32 = 0.0;
        let mut weapon_accuracies: Vec<NewWeaponAccuracy> = Vec::new();

        if caps.is_some() {
            let caps_unwrapped = caps.unwrap();

            default_accuracy = caps_unwrapped.get(1).unwrap().as_str().parse().unwrap_or(0.0);

            let family_str = caps_unwrapped.get(2).unwrap().as_str();

            for family in rx_family.captures_iter(family_str) {
                weapon_accuracies.push(NewWeaponAccuracy {
                    weapon_name: weapon_name.to_string(),
                    armor_family: family.get(1).unwrap().as_str().to_string(),
                    accuracy: family.get(2).unwrap().as_str().parse().unwrap_or(0.0),
                    damage: if let Some(d) = family.get(3) {
                        d.as_str().parse().unwrap_or(1.0)
                    } else {
                        1.0
                    }
                });
            }
        } else {
            println!("No weapon accuracies found for: {}!", weapon_name);
        }

        Self {
            default_accuracy,
            weapon_accuracies
        }
    }
}
