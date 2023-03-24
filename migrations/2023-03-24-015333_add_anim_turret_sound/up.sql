-- Your SQL goes here
CREATE TABLE weapon_turret_sound (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	weapon_name TEXT NOT NULL,
    anim_turret_sound TEXT NOT NULL,
	CONSTRAINT weapon_results_FK FOREIGN KEY (weapon_name) REFERENCES weapons(name) ON DELETE CASCADE ON UPDATE CASCADE,
    UNIQUE(weapon_name)
);
