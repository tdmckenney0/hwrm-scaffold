-- Your SQL goes here
ALTER TABLE `weapons` ADD COLUMN `field_penetration` INTEGER NOT NULL DEFAULT 0;
ALTER TABLE `weapons` ADD COLUMN `default_penetration` FLOAT NOT NULL DEFAULT 0.0;
