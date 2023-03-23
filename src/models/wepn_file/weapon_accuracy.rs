use regex::Regex;
use diesel::prelude::*;
use crate::schema::{weapon_accuracy};
use super::weapon::Weapon;

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
