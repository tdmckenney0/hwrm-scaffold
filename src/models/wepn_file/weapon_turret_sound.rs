use regex::Regex;
use diesel::prelude::*;
use crate::schema::{weapon_turret_sound};
use super::weapon::Weapon;

///
/// Regex's
///

/// Find the `addAnimTurretSound(...)` function call in a string, parse into groups.
pub fn add_anim_turret_regex() -> Regex {
    Regex::new(r#"addAnimTurretSound\(NewWeaponType\s*,\s*"(.*)"\s*\)"#).unwrap()
}

///
/// Weapon Turret Sound
///
#[derive(Queryable, Selectable, Identifiable, Associations, Debug)]
#[diesel(belongs_to(Weapon, foreign_key = weapon_name))]
#[diesel(table_name = weapon_turret_sound)]
pub struct WeaponTurretSound {
    pub id: i32,
    pub weapon_name: String,
    pub anim_turret_sound: String
}


///
/// NewWeaponTurretSound
///

/// Class Ready to be Inserted via Diesel
#[derive(Default, Insertable, Debug)]
#[diesel(table_name = weapon_turret_sound)]
pub struct NewWeaponTurretSound {
    pub weapon_name: String,
    pub anim_turret_sound: String
}

impl NewWeaponTurretSound {
    /// Parse the `addAnimTurretSound(...)` lua call into an instance.
    /// This whole process needs to be optimized at some point.
    pub fn from_string(weapon_name: &String, body: &String) -> Option<Self> {
        let rx = add_anim_turret_regex();

        if let Some(caps) = rx.captures(body) {
            Some(Self {
                weapon_name: weapon_name.to_string(),
                anim_turret_sound: caps.get(1).unwrap().as_str().to_string()
            })
        } else {
            None
        }
    }
}
