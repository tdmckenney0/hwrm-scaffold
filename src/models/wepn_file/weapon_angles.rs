use regex::Regex;
use diesel::prelude::*;
use crate::schema::{weapon_angles};
use super::weapon::Weapon;

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
    pub fn from_string(weapon_name: &String, body: &String) -> Self {
        let rx = set_angles_regex();
        let caps = rx.captures(body).unwrap();

        NewWeaponAngles {
            weapon_name: weapon_name.to_string(),
            firing_cone: caps.get(1).unwrap().as_str().parse().unwrap_or(0.0),
            min_azimuth: caps.get(2).unwrap().as_str().parse().unwrap_or(0.0),
            max_azimuth: caps.get(3).unwrap().as_str().parse().unwrap_or(0.0),
            min_declination: caps.get(4).unwrap().as_str().parse().unwrap_or(0.0),
            max_declination: caps.get(5).unwrap().as_str().parse().unwrap_or(0.0)
        }
    }
}
