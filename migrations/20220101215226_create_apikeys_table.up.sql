-- Add up migration script here
CREATE TABLE apikeys (
    id SERIAL PRIMARY KEY NOT NULL,
    key_owner TEXT NOT NULL,
    apikey VARCHAR(32) NOT NULL,
    expires_at TIMESTAMP NOT NULL
);