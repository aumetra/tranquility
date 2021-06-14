ALTER TABLE actors 
    ALTER created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC', 
    ALTER updated_at TYPE TIMESTAMPTZ USING updated_at AT TIME ZONE 'UTC';

ALTER TABLE objects 
    ALTER created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC', 
    ALTER updated_at TYPE TIMESTAMPTZ USING updated_at AT TIME ZONE 'UTC';

ALTER TABLE oauth_applications 
    ALTER created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC', 
    ALTER updated_at TYPE TIMESTAMPTZ USING updated_at AT TIME ZONE 'UTC';

ALTER TABLE oauth_authorizations 
    ALTER created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC', 
    ALTER updated_at TYPE TIMESTAMPTZ USING updated_at AT TIME ZONE 'UTC', 
    ALTER valid_until TYPE TIMESTAMPTZ USING valid_until AT TIME ZONE 'UTC';

ALTER TABLE oauth_tokens 
    ALTER created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC', 
    ALTER updated_at TYPE TIMESTAMPTZ USING updated_at AT TIME ZONE 'UTC', 
    ALTER valid_until TYPE TIMESTAMPTZ USING valid_until AT TIME ZONE 'UTC';
