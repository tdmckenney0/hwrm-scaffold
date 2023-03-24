use regex::Regex;
use diesel::prelude::*;
use crate::schema::{weapon_misc};
use super::weapon::Weapon;
use std::fmt;

///
/// Regex's
///

/// Find the `setMiscValues(...)` function call in a string, parse into groups.
pub fn set_misc_values_regex() -> Regex {
    Regex::new(r"setMiscValues\(NewWeaponType\s*,\s*([0-9\.\-]*)\s*,\s*([0-9\.\-]*)\s*\)").unwrap()
}

///
/// Weapon Misc
///
#[derive(Queryable, Selectable, Identifiable, Associations, Debug)]
#[diesel(belongs_to(Weapon, foreign_key = weapon_name))]
#[diesel(table_name = weapon_misc)]
pub struct WeaponMisc {
    pub id: i32,
    pub weapon_name: String,
    pub recoil_distance: f32,
    pub slave_fire_delay: f32
}

impl fmt::Display for WeaponMisc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "setMiscValues(NewWeaponType,{},{});",
            self.recoil_distance,
            self.slave_fire_delay
        )
    }
}

///
/// NewWeaponMisc
///

/// Class Ready to be Inserted via Diesel
#[derive(Default, Insertable, Debug)]
#[diesel(table_name = weapon_misc)]
pub struct NewWeaponMisc {
    pub weapon_name: String,
    pub recoil_distance: f32,
    pub slave_fire_delay: f32
}

impl NewWeaponMisc {
    /// Parse the `setMiscValues(...)` lua call into an instance.
    /// This whole process needs to be optimized at some point.
    pub fn from_string(weapon_name: &String, body: &String) -> Option<Self> {
        let rx = set_misc_values_regex();

        if let Some(caps) = rx.captures(body) {
            Some(Self {
                weapon_name: weapon_name.to_string(),
                recoil_distance: caps.get(1).unwrap().as_str().parse().unwrap_or(0.0),
                slave_fire_delay: caps.get(2).unwrap().as_str().parse().unwrap_or(0.0)
            })
        } else {
            None
        }
    }
}
