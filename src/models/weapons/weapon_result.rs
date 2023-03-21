use std::fmt;
use diesel::prelude::*;
use crate::schema::{weapon_results};
use regex::Regex;
use weapon::Weapon;

///
/// Weapon Result
///
#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(Weapon))]
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
        let formatted_spawn_weapon_id = match self.spawn_weapon_id {
            Some(n) => n,
            None => "".to_string(),
        };

        write!(f, "AddWeaponResult(NewWeaponType, \"{}\", \"{}\", \"{}\", {}, {}, \"{}\");",
            self.condition,
            self.effect,
            self.target,
            self.minimum_effect,
            self.maximum_effect,
            formatted_spawn_weapon_id
        )
    }
}
