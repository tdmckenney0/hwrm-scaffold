-- weapon_results definition
CREATE TABLE weapon_results (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	weapon_name TEXT NOT NULL,
	"condition" TEXT NOT NULL DEFAULT ('Hit'),
	effect TEXT NOT NULL DEFAULT ('DamageHealth'),
	target TEXT NOT NULL DEFAULT ('Target'),
	minimum_effect FLOAT NOT NULL DEFAULT (0),
	maximum_effect FLOAT NOT NULL DEFAULT (0),
	spawn_weapon_name TEXT DEFAULT NULL,
	CONSTRAINT weapon_results_FK FOREIGN KEY (weapon_name) REFERENCES weapons(name) ON DELETE CASCADE ON UPDATE CASCADE
);
