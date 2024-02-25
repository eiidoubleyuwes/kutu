-- Your SQL goes here
-- Create a database for users called watu
CREATE TABLE watu (
    id INTEGER NOT NULL PRIMARY KEY,
    email TEXT NOT NULL,
    funguo TEXT NOT NULL,
    vault TEXT NOT NULL,
    UNIQUE(email)
);