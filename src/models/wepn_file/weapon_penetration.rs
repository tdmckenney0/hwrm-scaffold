use regex::Regex;
use diesel::prelude::*;
use crate::schema::{weapon_penetrations};
use super::weapon::Weapon;

///
/// Regex's
///

/// Find the `setPenetration(...)` function call in a string, parse into groups.
pub fn set_penetration_regex() -> Regex {
    Regex::new(r"setPenetration\(NewWeaponType\s*,\s*([0-9]*)\s*,\s*([0-9\.]*)\s*,\s*([\sA-za-z0-9=\.\{\},]*)\s*\)").unwrap()
}

/// Find the armor family arguments in `setPenetration(...)`
pub fn armor_family_regex() -> Regex {
    Regex::new(r"\{\s*(\w*)\s*=\s*(\d*\.*\d*)\s*,*\s*\}").unwrap()
}

///
/// Weapon Penetration
///
#[derive(Queryable, Selectable, Identifiable, Associations, Debug)]
#[diesel(belongs_to(Weapon, foreign_key = weapon_name))]
#[diesel(table_name = weapon_penetrations)]
pub struct WeaponPenetration {
    pub id: i32,
    pub weapon_name: String,
    pub armor_family: String,
    pub penetration: f32
}

///
/// NewWeaponPenetration
///

/// Class Ready to be inserted via Diesel.
#[derive(Default, Insertable, Debug)]
#[diesel(table_name = weapon_penetrations)]
pub struct NewWeaponPenetration {
    pub weapon_name: String,
    pub armor_family: String,
    pub penetration: f32
}

/// Collection of `NewWeaponPenetration` with helper methods. Has additional arguments from the `setPenetration` lua call.
#[derive(Debug)]
pub struct NewWeaponPenetrationCollection {
    pub field_penetration: i32,
    pub default_penetration: f32,
    pub weapon_penetrations: Vec<NewWeaponPenetration>
}

impl NewWeaponPenetrationCollection {
    /// Parse the `setPenetration(...)` lua call into an instance.
    /// This whole process needs to be optimized at some point.
    pub fn from_string(weapon_name: &String, body: &String) -> Self {
        let rx = set_penetration_regex();
        let rx_family = armor_family_regex();

        let caps = rx.captures(body);

        let mut field_penetration: i32 = 0;
        let mut default_penetration: f32 = 0.0;
        let mut weapon_penetrations: Vec<NewWeaponPenetration> = Vec::new();

        if caps.is_some() {
            let caps_unwrapped = caps.unwrap();

            field_penetration = caps_unwrapped.get(1).unwrap().as_str().parse().unwrap_or(0);
            default_penetration = caps_unwrapped.get(2).unwrap().as_str().parse().unwrap_or(0.0);

            let family_str = caps_unwrapped.get(3).unwrap().as_str();

            for family in rx_family.captures_iter(family_str) {
                weapon_penetrations.push(NewWeaponPenetration {
                    weapon_name: weapon_name.to_string(),
                    armor_family: family.get(1).unwrap().as_str().to_string(),
                    penetration: family.get(2).unwrap().as_str().parse().unwrap_or(0.0)
                });
            }
        } else {
            println!("No weapon penetrations found for: {}!", weapon_name);
        }

        Self {
            field_penetration,
            default_penetration,
            weapon_penetrations
        }
    }
}
