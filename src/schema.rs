// @generated automatically by Diesel CLI.

diesel::table! {
    weapon_penetrations (id) {
        id -> Integer,
        weapon_name -> Text,
        armor_family -> Text,
        penetration -> Float,
    }
}

diesel::table! {
    weapon_results (id) {
        id -> Integer,
        weapon_name -> Text,
        condition -> Text,
        effect -> Text,
        target -> Text,
        minimum_effect -> Float,
        maximum_effect -> Float,
        spawn_weapon_name -> Nullable<Text>,
    }
}

diesel::table! {
    weapons (name) {
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
        field_penetration -> Integer,
        default_penetration -> Float,
    }
}

diesel::joinable!(weapon_penetrations -> weapons (weapon_name));
diesel::joinable!(weapon_results -> weapons (weapon_name));

diesel::allow_tables_to_appear_in_same_query!(
    weapon_penetrations,
    weapon_results,
    weapons,
);
