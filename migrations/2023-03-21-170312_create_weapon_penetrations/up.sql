-- Your SQL goes here
CREATE TABLE weapon_penetrations (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	weapon_name TEXT NOT NULL,
	armor_family TEXT NOT NULL,
    penetration FLOAT NOT NULL,
	CONSTRAINT weapon_results_FK FOREIGN KEY (weapon_name) REFERENCES weapons(name) ON DELETE CASCADE ON UPDATE CASCADE,
    UNIQUE(weapon_name, armor_family)
);
