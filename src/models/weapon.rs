use diesel::prelude::*;
use crate::schema::{weapons, weapon_results};
use regex::Regex;

/// Diesel Models.
#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = weapons)]
pub struct Weapon {
    pub id: i32,
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
}

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(Weapon))]
#[diesel(table_name = weapon_results)]
pub struct WeaponResult {
    pub id: i32,
    pub weapon_id: i32,
    pub condition: String,
    pub effect: String,
    pub target: String,
    pub minimum_effect: f32,
    pub maximum_effect: f32,
    pub spawn_weapon_id: Option<i32>
}

/// Parse Result from the `StartWeaponConfig(...)` lua function call.
/// Can be inserted into `weapons` table.
#[derive(Default, Insertable)]
#[diesel(table_name = weapons)]
pub struct StartWeaponConfig {
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
    pub instant_hit_threshold: i32
}

impl StartWeaponConfig {
    pub fn new (name: String, start_weapon_config: String) -> Self {
       let rx = Regex::new(r"^StartWeaponConfig\(NewWeaponType,\s*(.*)\)").unwrap();

        let args = match rx.captures(&start_weapon_config).unwrap().get(1) {
            Some(x) => x.as_str(),
            None => ""
        };

        let split: Vec<&str> = args.split(",").collect();

        Self {
            name,
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
            instant_hit_threshold: split[24].parse().unwrap()
        }
    }

    pub fn to_string() -> String {
        "".to_string()
    }
}
