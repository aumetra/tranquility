-- Add migration script here
ALTER TABLE actors ADD COLUMN remote BOOLEAN NOT NULL DEFAULT FALSE;
