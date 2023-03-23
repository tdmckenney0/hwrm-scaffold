-- Your SQL goes here
CREATE TABLE weapon_angles (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	weapon_name TEXT NOT NULL,
	firing_cone FLOAT NOT NULL DEFAULT 0.0,
    min_azimuth FLOAT NOT NULL DEFAULT 0.0,
    max_azimuth FLOAT NOT NULL DEFAULT 0.0,
    min_declination FLOAT NOT NULL DEFAULT 0.0,
    max_declination FLOAT NOT NULL DEFAULT 0.0,
	CONSTRAINT weapon_results_FK FOREIGN KEY (weapon_name) REFERENCES weapons(name) ON DELETE CASCADE ON UPDATE CASCADE,
    UNIQUE(weapon_name)
);
