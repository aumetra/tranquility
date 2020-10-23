CREATE UNIQUE INDEX username_local_constraint ON actors (username) WHERE NOT remote;
