use std::fmt;
use diesel::prelude::*;
use crate::schema::{weapon_results};
use regex::Regex;
use super::weapon::Weapon;
use std::collections::HashMap;

///
/// Regex's
///
pub fn add_weapon_result_regex() -> Regex {
    Regex::new(r"AddWeaponResult\(NewWeaponType,\s*(.*)\)").unwrap()
}

///
/// Weapon Result
///
#[derive(Queryable, Selectable, Identifiable, Associations, Debug)]
#[diesel(belongs_to(Weapon, foreign_key = weapon_name))]
#[diesel(table_name = weapon_results)]
pub struct WeaponResult {
    pub id: i32,
    pub weapon_name: String,
    pub condition: String,
    pub effect: String,
    pub target: String,
    pub minimum_effect: f32,
    pub maximum_effect: f32,
    pub spawn_weapon_name: Option<String>
}

impl fmt::Display for WeaponResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let spawn_weapon_name = self.spawn_weapon_name.as_deref().unwrap_or("");

        write!(f, "AddWeaponResult(NewWeaponType, \"{}\", \"{}\", \"{}\", {}, {}, \"{}\");",
            self.condition,
            self.effect,
            self.target,
            self.minimum_effect,
            self.maximum_effect,
            spawn_weapon_name
        )
    }
}

/// Array of `WeaponResult` with helper methods.
#[derive(Debug)]
pub struct WeaponResultCollection {
    pub weapon_results: Vec<WeaponResult>
}

impl fmt::Display for WeaponResultCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let format = self.weapon_results
                        .iter()
                        .map(|wr| wr.to_string())
                        .collect::<Vec<String>>()
                        .join("\r\n");

        write!(f, "{}", format)
    }
}

impl WeaponResultCollection {
    /// Return a new empty instance
    pub fn new() -> Self {
        Self {
            weapon_results: Vec::new()
        }
    }

    /// Get weapon results for a specific weapon name.
    pub fn get_for_weapon(connection: &mut SqliteConnection, name: &String) -> Self {
        use crate::schema::weapon_results::dsl::*;

        let vec = weapon_results
                    .filter(weapon_name.eq(name))
                    .load::<WeaponResult>(connection)
                    .expect("Error loading weapon results!");

        Self {
            weapon_results: vec
        }
    }

    /// Get weapon results for a list of weapon names.
    pub fn get_for_weapons(connection: &mut SqliteConnection, names: &Vec<String>) -> Self {
        use crate::schema::weapon_results::dsl::*;

        let vec = weapon_results
                    .filter(weapon_name.eq_any(names))
                    .load::<WeaponResult>(connection)
                    .expect("Error loading weapon results!");

        Self {
            weapon_results: vec
        }
    }

    /// Consume the collection, divide into HashMap by `weapon_name`
    pub fn key_by_weapon_name(self) -> HashMap<String, Self> {
        let mut weapon_results: Vec<WeaponResult> = self.weapon_results;
        let mut map = HashMap::new();

        while let Some(wr) = weapon_results.pop() {
            if !map.contains_key(&wr.weapon_name) {
                map.insert(wr.weapon_name.to_string(), Self {
                    weapon_results: vec![wr]
                });
            } else {
                map
                    .get_mut(&wr.weapon_name)
                    .expect("Could not find key??")
                    .weapon_results.push(wr);
            }
        }

        map
    }
}

///
/// NewWeaponResult
///

/// Class Ready to be inserted via Diesel.
#[derive(Default, Insertable, Debug)]
#[diesel(table_name = weapon_results)]
pub struct NewWeaponResult {
    pub weapon_name: String,
    pub condition: String,
    pub effect: String,
    pub target: String,
    pub minimum_effect: f32,
    pub maximum_effect: f32,
    pub spawn_weapon_name: Option<String>
}

/// Array of `NewWeaponResult` with helper methods.
#[derive(Debug)]
pub struct NewWeaponResultCollection {
    pub weapon_results: Vec<NewWeaponResult>
}

impl NewWeaponResultCollection {
    /// Parse the `AddWeaponResult(...)` lua calls into an instance.
    /// This whole process needs to be optimized at some point.
    pub fn from_string(weapon_name: &String, body: &String) -> Self {
        let rx = add_weapon_result_regex();
        let mut weapon_results: Vec<NewWeaponResult> = Vec::new();

        for caps in rx.captures_iter(&body) {
            let split: Vec<&str> = caps[1]
                                        .split(",")
                                        .collect::<Vec<&str>>()
                                        .iter()
                                        .map(|x| x.trim())
                                        .collect();

            weapon_results.push(NewWeaponResult {
                weapon_name: weapon_name.to_string(),
                condition: split[0].to_string().replace("\"", ""),
                effect: split[1].to_string().replace("\"", ""),
                target: split[2].to_string().replace("\"", ""),
                minimum_effect: split[3].parse().unwrap_or_default(),
                maximum_effect: split[4].parse().unwrap_or_default(),
                spawn_weapon_name: Some(split[5].to_string().replace("\"", ""))
            })
        }

        Self {
            weapon_results
        }
    }
}
