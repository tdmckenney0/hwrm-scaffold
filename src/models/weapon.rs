use diesel::prelude::*;
use crate::schema::{weapons, weapon_results};

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
