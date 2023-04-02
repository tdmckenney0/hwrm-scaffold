use regex::Regex;
use diesel::prelude::*;
use crate::schema::{weapon_turret_sound};
use super::weapon::Weapon;
use std::fmt;
use std::collections::HashMap;

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

impl fmt::Display for WeaponTurretSound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "addAnimTurretSound(NewWeaponType,\"{}\");", self.anim_turret_sound)
    }
}

impl WeaponTurretSound {
    /// Get weapon turret sound for a specific weapon name. Possible that it can't be found.
    pub fn get_for_weapon(connection: &mut SqliteConnection, name: &String) -> Option<Self> {
        use crate::schema::weapon_turret_sound::dsl::*;

        let res = weapon_turret_sound
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

pub struct WeaponTurretSoundCollection {
    pub weapon_turret_sounds: HashMap<String, WeaponTurretSound>
}

impl WeaponTurretSoundCollection {
    /// Get weapon misc for a list of weapon names.
    pub fn get_for_weapons(connection: &mut SqliteConnection, names: &Vec<String>) -> Self {
        use crate::schema::weapon_turret_sound::dsl::*;

        let mut vec = weapon_turret_sound
                    .filter(weapon_name.eq_any(names))
                    .load::<WeaponTurretSound>(connection)
                    .expect("Error loading weapon turret sounds!");

        let mut map = HashMap::new();

        for sound in vec.drain(..) {
            map.insert(sound.weapon_name.to_string(), sound);
        }

        Self {
            weapon_turret_sounds: map
        }
    }
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
