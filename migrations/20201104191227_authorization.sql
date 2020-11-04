CREATE TABLE oauth_applications (
    id              UUID        PRIMARY KEY     DEFAULT uuid_generate_v4(),

    client_name     TEXT        NOT NULL,
    client_id       UUID        NOT NULL,
    client_secret   TEXT        NOT NULL,

    redirect_uris   TEXT        NOT NULL,
    scopes          TEXT        NOT NULL,
    website         TEXT        NOT NULL,

    created_at      TIMESTAMP   NOT NULL        DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP   NOT NULL        DEFAULT CURRENT_TIMESTAMP,

    UNIQUE (client_id)
);

CREATE TABLE oauth_authorizations (
    id              UUID        PRIMARY KEY     DEFAULT uuid_generate_v4(),

    application_id  UUID        NOT NULL        REFERENCES oauth_applications(id),
    actor_id        UUID        NOT NULL        REFERENCES actors(id),

    code            TEXT        NOT NULL,
    valid_until     TIMESTAMP   NOT NULL,

    created_at      TIMESTAMP   NOT NULL        DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP   NOT NULL        DEFAULT CURRENT_TIMESTAMP,

    UNIQUE (code)
);

CREATE TABLE oauth_tokens (
    id              UUID        PRIMARY KEY     DEFAULT uuid_generate_v4(),

    application_id  UUID        NOT NULL        REFERENCES oauth_applications(id),
    actor_id        UUID        NOT NULL        REFERENCES actors(id),

    access_token    TEXT        NOT NULL,
    refresh_token   TEXT,
    valid_until     TIMESTAMP   NOT NULL,

    created_at      TIMESTAMP   NOT NULL        DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP   NOT NULL        DEFAULT CURRENT_TIMESTAMP,

    UNIQUE (access_token),
    UNIQUE (refresh_token)
);

SELECT add_updated_at_trigger('oauth_applications');
SELECT add_updated_at_trigger('oauth_authorizations');
SELECT add_updated_at_trigger('oauth_tokens');
