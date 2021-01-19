ALTER TABLE objects DROP COLUMN url;
CREATE UNIQUE INDEX unique_object_id ON objects(((data->>'id')::TEXT));