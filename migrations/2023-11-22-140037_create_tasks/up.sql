-- Your SQL goes here
CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description TEXT,
    priority VARCHAR,
    completed BOOLEAN NOT NULL
);