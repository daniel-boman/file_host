-- Add down migration script here
DROP TABLE IF EXISTS files;
DROP INDEX IF EXISTS hash_idx;