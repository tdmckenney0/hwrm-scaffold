use regex::Regex;
use diesel::prelude::*;
use crate::schema::{weapon_misc};
use super::weapon::Weapon;
use std::fmt;
use std::collections::HashMap;

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

impl WeaponMisc {
    /// Get weapon misc for a specific weapon name. Possible that it can't be found.
    pub fn get_for_weapon(connection: &mut SqliteConnection, name: &String) -> Option<Self> {
        use crate::schema::weapon_misc::dsl::*;

        let res = weapon_misc
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

pub struct WeaponMiscCollection {
    pub weapon_misc: HashMap<String, WeaponMisc>
}

impl WeaponMiscCollection {
    /// Get weapon misc for a list of weapon names.
    pub fn get_for_weapons(connection: &mut SqliteConnection, names: &Vec<String>) -> Self {
        use crate::schema::weapon_misc::dsl::*;

        let mut vec = weapon_misc
                    .filter(weapon_name.eq_any(names))
                    .load::<WeaponMisc>(connection)
                    .expect("Error loading weapon misc!");

        let mut map = HashMap::new();

        for misc in vec.drain(..) {
            map.insert(misc.weapon_name.to_string(), misc);
        }

        Self {
            weapon_misc: map
        }
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
