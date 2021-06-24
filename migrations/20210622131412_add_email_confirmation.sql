ALTER TABLE actors 
    ADD COLUMN is_confirmed BOOLEAN NOT NULL DEFAULT TRUE;
ALTER TABLE actors
    ADD COLUMN confirmation_code TEXT;
