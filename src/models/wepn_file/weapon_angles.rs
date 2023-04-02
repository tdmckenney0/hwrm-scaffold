use regex::Regex;
use diesel::prelude::*;
use crate::schema::{weapon_angles};
use super::weapon::Weapon;
use std::fmt;
use std::collections::HashMap;

///
/// Regex's
///

/// Find the `setAngles(...)` function call in a string, parse into groups.
pub fn set_angles_regex() -> Regex {
    Regex::new(r"setAngles\(NewWeaponType\s*,\s*([0-9\.\-]*)\s*,\s*([0-9\.\-]*)\s*,\s*([0-9\.\-]*)\s*,\s*([0-9\.\-]*)\s*,\s*([0-9\.\-]*)\s*\)").unwrap()
}

///
/// Weapon Angle
///
#[derive(Queryable, Selectable, Identifiable, Associations, Debug)]
#[diesel(belongs_to(Weapon, foreign_key = weapon_name))]
#[diesel(table_name = weapon_angles)]
pub struct WeaponAngles {
    pub id: i32,
    pub weapon_name: String,
    pub firing_cone: f32,
    pub min_azimuth: f32,
    pub max_azimuth: f32,
    pub min_declination: f32,
    pub max_declination: f32
}

impl fmt::Display for WeaponAngles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "setAngles(NewWeaponType,{},{},{},{},{});",
            self.firing_cone,
            self.min_azimuth,
            self.max_azimuth,
            self.min_declination,
            self.max_declination
        )
    }
}

impl WeaponAngles {
    /// Get weapon angles for a specific weapon name. Possible that it can't be found.
    pub fn get_for_weapon(connection: &mut SqliteConnection, name: &String) -> Option<Self> {
        use crate::schema::weapon_angles::dsl::*;

        let res = weapon_angles
                    .filter(weapon_name.eq(name))
                    .first(connection)
                    .optional();

        if let Ok(op) = res {
            op
        } else {
            None
        }
    }
}

pub struct WeaponAnglesCollection {
    pub weapon_angles: HashMap<String, WeaponAngles>
}

impl WeaponAnglesCollection {
    /// Get weapon angles for a list of weapon names.
    pub fn get_for_weapons(connection: &mut SqliteConnection, names: &Vec<String>) -> Self {
        use crate::schema::weapon_angles::dsl::*;

        let mut vec = weapon_angles
                    .filter(weapon_name.eq_any(names))
                    .load::<WeaponAngles>(connection)
                    .expect("Error loading weapon angles!");

        let mut map = HashMap::new();

        for angles in vec.drain(..) {
            map.insert(angles.weapon_name.to_string(), angles);
        }

        Self {
            weapon_angles: map
        }
    }
}

///
/// NewWeaponAngles
///

/// Class Ready to be Inserted via Diesel
#[derive(Default, Insertable, Debug)]
#[diesel(table_name = weapon_angles)]
pub struct NewWeaponAngles {
    pub weapon_name: String,
    pub firing_cone: f32,
    pub min_azimuth: f32,
    pub max_azimuth: f32,
    pub min_declination: f32,
    pub max_declination: f32
}

impl NewWeaponAngles {
    /// Parse the `setAngles(...)` lua call into an instance.
    /// This whole process needs to be optimized at some point.
    pub fn from_string(weapon_name: &String, body: &String) -> Option<Self> {
        let rx = set_angles_regex();
        let some_caps = rx.captures(body);

        if let Some(caps) = some_caps {
            Some(NewWeaponAngles {
                weapon_name: weapon_name.to_string(),
                firing_cone: caps.get(1).unwrap().as_str().parse().unwrap_or(0.0),
                min_azimuth: caps.get(2).unwrap().as_str().parse().unwrap_or(0.0),
                max_azimuth: caps.get(3).unwrap().as_str().parse().unwrap_or(0.0),
                min_declination: caps.get(4).unwrap().as_str().parse().unwrap_or(0.0),
                max_declination: caps.get(5).unwrap().as_str().parse().unwrap_or(0.0)
            })
        } else {
            None
        }
    }
}
