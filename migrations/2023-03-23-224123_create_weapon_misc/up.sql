-- Your SQL goes here
CREATE TABLE weapon_misc (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	weapon_name TEXT NOT NULL,
	recoil_distance FLOAT NOT NULL DEFAULT 0.0,
    slave_fire_delay FLOAT NOT NULL DEFAULT 0.0,
	CONSTRAINT weapon_results_FK FOREIGN KEY (weapon_name) REFERENCES weapons(name) ON DELETE CASCADE ON UPDATE CASCADE,
    UNIQUE(weapon_name)
);
