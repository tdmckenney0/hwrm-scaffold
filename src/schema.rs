// @generated automatically by Diesel CLI.

diesel::table! {
    weapons (id) {
        id -> Integer,
        name -> Text,
        weapon_type -> Text,
        weapon_fire_type -> Text,
        weapon_fire_name -> Text,
        activation -> Text,
        fire_speed -> Float,
        fire_range -> Float,
        fire_radius -> Float,
        fire_lifetime -> Float,
        fire_anticipation_time -> Float,
        fire_axis -> Integer,
        max_effects_spawned -> Integer,
        lead_target -> Integer,
        check_line_of_fire -> Integer,
        fire_time -> Float,
        burst_fire_time -> Float,
        burst_wait_time -> Float,
        shoot_at_secondaries -> Integer,
        shoot_at_surroundings -> Integer,
        max_azimuth_speed -> Float,
        max_declination_speed -> Float,
        speed_multiplier -> Float,
        shield_penetration -> Text,
        track_targets_outside_range -> Integer,
        wait_for_code_red -> Float,
        instant_hit_threshold -> Integer,
    }
}
