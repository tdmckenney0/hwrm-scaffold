use std::fmt;
use diesel::prelude::*;
use crate::schema::{weapons};
use regex::Regex;
use std::collections::HashMap;

///
/// Regex's
///
pub fn start_weapon_config_regex() -> Regex {
    Regex::new(r"StartWeaponConfig\(NewWeaponType,\s*(.*)\)").unwrap()
}

///
/// Weapon
///
#[derive(Queryable, Selectable, Identifiable, Default, Insertable, Debug)]
#[diesel(table_name = weapons)]
#[diesel(primary_key(name))]
pub struct Weapon {
    pub name: String,
    pub weapon_type: String,
    pub weapon_fire_type: String,
    pub weapon_fire_name: String,
    pub activation: String,
    pub fire_speed: f32,
    pub fire_range: f32,
    pub fire_radius: f32,
    pub fire_lifetime: f32,
    pub fire_anticipation_time: f32,
    pub fire_axis: i32,
    pub max_effects_spawned: i32,
    pub lead_target: i32,
    pub check_line_of_fire: i32,
    pub fire_time: f32,
    pub burst_fire_time: f32,
    pub burst_wait_time: f32,
    pub shoot_at_secondaries: i32,
    pub shoot_at_surroundings: i32,
    pub max_azimuth_speed: f32,
    pub max_declination_speed: f32,
    pub speed_multiplier: f32,
    pub shield_penetration: String,
    pub track_targets_outside_range: i32,
    pub wait_for_code_red: f32,
    pub instant_hit_threshold: i32,

    /// These are set later by the `setPenetration` call.
    pub field_penetration: i32,
    pub default_penetration: f32,

    /// These are set later by the `setAccuracy` call.
    pub default_accuracy: f32
}

impl fmt::Display for Weapon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StartWeaponConfig(NewWeaponType,\"{}\",\"{}\",\"{}\",\"{}\",{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},\"{}\",{},{},{});",
            self.weapon_type,
            self.weapon_fire_type,
            self.weapon_fire_name,
            self.activation,
            self.fire_speed,
            self.fire_range,
            self.fire_radius,
            self.fire_lifetime,
            self.fire_anticipation_time,
            self.fire_axis,
            self.max_effects_spawned,
            self.lead_target,
            self.check_line_of_fire,
            self.fire_time,
            self.burst_fire_time,
            self.burst_wait_time,
            self.shoot_at_secondaries,
            self.shoot_at_surroundings,
            self.max_azimuth_speed,
            self.max_declination_speed,
            self.speed_multiplier,
            self.shield_penetration,
            self.track_targets_outside_range,
            self.wait_for_code_red,
            self.instant_hit_threshold
        )
    }
}

impl Weapon {
    /// Create a `Weapon` Model from a `name` and String `body`.
    pub fn from_string(name: &String, body: &String) -> Self {
        let rx: Regex = start_weapon_config_regex();

        let args = match rx.captures(&body).unwrap().get(1) {
            Some(x) => x.as_str(),
            None => ""
        };

        let split: Vec<&str> = args.split(",").collect::<Vec<&str>>().iter().map(|x| x.trim()).collect();

        Self {
            name: name.to_string(),
            weapon_type: split[0].to_string().replace("\"", ""),
            weapon_fire_type: split[1].to_string().replace("\"", ""),
            weapon_fire_name: split[2].to_string().replace("\"", ""),
            activation: split[3].to_string().replace("\"", ""),
            fire_speed: split[4].parse().unwrap(),
            fire_range: split[5].parse().unwrap(),
            fire_radius: split[6].parse().unwrap(),
            fire_lifetime: split[7].parse().unwrap(),
            fire_anticipation_time: split[8].parse().unwrap(),
            fire_axis: split[9].parse().unwrap(),
            max_effects_spawned: split[10].parse().unwrap(),
            lead_target: split[11].parse().unwrap(),
            check_line_of_fire: split[12].parse().unwrap(),
            fire_time: split[13].parse().unwrap(),
            burst_fire_time: split[14].parse().unwrap(),
            burst_wait_time: split[15].parse().unwrap(),
            shoot_at_secondaries: split[16].parse().unwrap(),
            shoot_at_surroundings: split[17].parse().unwrap(),
            max_azimuth_speed: split[18].parse().unwrap(),
            max_declination_speed: split[19].parse().unwrap(),
            speed_multiplier: split[20].parse().unwrap(),
            shield_penetration: split[21].to_string().replace("\"", ""),
            track_targets_outside_range: split[22].parse().unwrap(),
            wait_for_code_red: split[23].parse().unwrap(),
            instant_hit_threshold: split[24].parse().unwrap(),
            field_penetration: 0,
            default_penetration: 0.0,
            default_accuracy: 1.0
        }
    }

    /// Get the Weapon File for a specific weapon name. Possible that it can't be found.
    pub fn get_for_weapon(connection: &mut SqliteConnection, weapon_name: &String) -> Option<Self> {
        use crate::schema::weapons::dsl::*;

        let res = weapons
                    .filter(name.eq(weapon_name))
                    .first(connection)
                    .optional();

        if let Ok(op) = res {
            op
        } else {
            None
        }
    }
}

pub struct WeaponCollection {
    pub weapons: HashMap<String, Weapon>
}

impl WeaponCollection {
    /// Get all Weapons from the Database.
    pub fn get_all_weapons(connection: &mut SqliteConnection) -> Self {
        use crate::schema::weapons::dsl::*;

        let mut weapons_vec = weapons
                            .load::<Weapon>(connection)
                            .expect("Could not load weapons table!");

        let mut weapons_map = HashMap::new();

        while let Some(w) = weapons_vec.pop() {
            weapons_map.insert(w.name.to_string(), w);
        }

        Self {
            weapons: weapons_map
        }
    }

    /// Get weapons from a list of weapon_name Strings
    pub fn get_weapons_from_names(connection: &mut SqliteConnection, weapon_names: Vec<String>) -> Self {
        use crate::schema::weapons::dsl::*;

        let mut weapons_vec = weapons
                            .filter(name.eq_any(weapon_names))
                            .load::<Weapon>(connection)
                            .expect("Could not load weapons table!");

        let mut weapons_map = HashMap::new();

        while let Some(w) = weapons_vec.pop() {
            weapons_map.insert(w.name.to_string(), w);
        }

        Self {
            weapons: weapons_map
        }
    }

    /// Get list of Names
    pub fn get_names(&self) -> Vec<String> {
        self.weapons.keys().map(|n| n.to_string()).collect()
    }
}
