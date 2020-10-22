ALTER TABLE activities ADD COLUMN url TEXT NOT NULL;
ALTER TABLE activities ADD CONSTRAINT unique_url_constraint UNIQUE (url);
