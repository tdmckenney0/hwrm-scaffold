-- Your SQL goes here
CREATE TABLE weapons (
	id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
	name TEXT NOT NULL DEFAULT "hgn_dummyfrigategun",
	weapon_type TEXT NOT NULL DEFAULT "Gimble",
	weapon_fire_type TEXT NOT NULL DEFAULT "Bullet",
	weapon_fire_name TEXT NOT NULL DEFAULT "kinetic_rapid",
	activation TEXT NOT NULL DEFAULT "Normal",
	fire_speed FLOAT NOT NULL DEFAULT 0,
	fire_range FLOAT NOT NULL DEFAULT 0,
	fire_radius FLOAT NOT NULL DEFAULT 0,
	fire_lifetime FLOAT NOT NULL DEFAULT 0,
	fire_anticipation_time FLOAT NOT NULL DEFAULT 0,
	fire_axis  INTEGER NOT NULL DEFAULT 0,
	max_effects_spawned  INTEGER NOT NULL DEFAULT 0,
	lead_target  INTEGER NOT NULL DEFAULT 0,
	check_line_of_fire  INTEGER NOT NULL DEFAULT 0,
	fire_time FLOAT NOT NULL DEFAULT 0,
	burst_fire_time FLOAT NOT NULL DEFAULT 0,
	burst_wait_time FLOAT NOT NULL DEFAULT 0,
	shoot_at_secondaries  INTEGER NOT NULL DEFAULT 0,
	shoot_at_surroundings  INTEGER NOT NULL DEFAULT 0,
	max_azimuth_speed FLOAT NOT NULL DEFAULT 0,
	max_declination_speed FLOAT NOT NULL DEFAULT 0,
	speed_multiplier FLOAT NOT NULL DEFAULT 0,
	shield_penetration TEXT NOT NULL DEFAULT "Normal",
	track_targets_outside_range  INTEGER NOT NULL DEFAULT 0,
	wait_for_code_red FLOAT NOT NULL DEFAULT 0,
	instant_hit_threshold  INTEGER NOT NULL DEFAULT 0
);

CREATE UNIQUE INDEX weapons_name_IDX ON weapons (name);