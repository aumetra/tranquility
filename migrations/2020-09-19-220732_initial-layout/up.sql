-- Activate the UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE actors (
    id              UUID    PRIMARY KEY DEFAULT uuid_generate_v4(),
    username        TEXT    NOT NULL,
    email           TEXT,
    password_hash   TEXT,

    actor           JSONB   NOT NULL,
    url             TEXT    NOT NULL,

    created_at      TIMESTAMP           DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP           DEFAULT CURRENT_TIMESTAMP,

    UNIQUE          (url),
    UNIQUE          (email)
);

CREATE TABLE activities (
    id              UUID    PRIMARY KEY DEFAULT uuid_generate_v4(),
    owner_id        UUID    NOT NULL    REFERENCES  actors(id),

    data            JSONB   NOT NULL,

    created_at      TIMESTAMP           DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP           DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('actors');
SELECT diesel_manage_updated_at('activities');
