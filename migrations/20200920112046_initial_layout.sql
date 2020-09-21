-- Activate the UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE actors (
    id              UUID    PRIMARY KEY     DEFAULT uuid_generate_v4(),
    username        TEXT    NOT NULL,
    email           TEXT,
    password_hash   TEXT,
    private_key     TEXT,

    actor           JSONB   NOT NULL,

    created_at      TIMESTAMP   NOT NULL    DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP   NOT NULL    DEFAULT CURRENT_TIMESTAMP,

    UNIQUE          (email)
);

CREATE TABLE activities (
    id              UUID    PRIMARY KEY     DEFAULT uuid_generate_v4(),
    owner_id        UUID    NOT NULL        REFERENCES  actors(id),

    data            JSONB   NOT NULL,

    created_at      TIMESTAMP   NOT NULL    DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP   NOT NULL    DEFAULT CURRENT_TIMESTAMP
);

CREATE OR REPLACE FUNCTION add_updated_at_trigger(_table REGCLASS) RETURNS VOID AS 
$$
    BEGIN
        EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s FOR EACH ROW EXECUTE PROCEDURE set_updated_at()', _table);
    END; 
$$ 
LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION set_updated_at() RETURNS TRIGGER AS 
$$
    BEGIN
        IF (NEW IS DISTINCT FROM OLD AND NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at) 
        THEN
            NEW.updated_at := CURRENT_TIMESTAMP;
        END IF;
        RETURN NEW;
    END; 
$$ 
LANGUAGE plpgsql;

SELECT add_updated_at_trigger('actors');
SELECT add_updated_at_trigger('activities');
