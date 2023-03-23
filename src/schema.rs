// @generated automatically by Diesel CLI.

diesel::table! {
    weapon_accuracy (id) {
        id -> Integer,
        weapon_name -> Text,
        armor_family -> Text,
        accuracy -> Float,
        damage -> Float,
    }
}

diesel::table! {
    weapon_angles (id) {
        id -> Integer,
        weapon_name -> Text,
        firing_cone -> Float,
        min_azimuth -> Float,
        max_azimuth -> Float,
        min_declination -> Float,
        max_declination -> Float,
    }
}

diesel::table! {
    weapon_misc (id) {
        id -> Integer,
        weapon_name -> Text,
        recoil_distance -> Float,
        slave_fire_delay -> Float,
    }
}

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
        default_accuracy -> Float,
    }
}

diesel::joinable!(weapon_accuracy -> weapons (weapon_name));
diesel::joinable!(weapon_angles -> weapons (weapon_name));
diesel::joinable!(weapon_misc -> weapons (weapon_name));
diesel::joinable!(weapon_penetrations -> weapons (weapon_name));
diesel::joinable!(weapon_results -> weapons (weapon_name));

diesel::allow_tables_to_appear_in_same_query!(
    weapon_accuracy,
    weapon_angles,
    weapon_misc,
    weapon_penetrations,
    weapon_results,
    weapons,
);
