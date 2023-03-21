-- This file should undo anything in `up.sql`
-- Your SQL goes here
ALTER TABLE `weapons` DROP COLUMN `field_penetration`;
ALTER TABLE `weapons` DROP COLUMN `default_penetration`;
