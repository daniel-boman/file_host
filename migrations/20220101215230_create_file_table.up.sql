-- Add up migration script here
-- file_type: 0=IMAGE 1=VIDEO 2=OTHER
CREATE TABLE files (
    id VARCHAR PRIMARY KEY,
    file_name VARCHAR NOT NULL,
    file_hash CHAR(64) NOT NULL,
    file_type SMALLINT CHECK(file_type IN (0, 1, 2)) NOT NULL DEFAULT 2, 
    file_size INTEGER NOT NULL,
    uploader CHAR(64) NOT NULL,
    upload_date TIMESTAMPTZ NOT NULL
);

CREATE UNIQUE INDEX hash_idx ON files(file_hash);