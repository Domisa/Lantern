-- Add migration script here
CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    date TEXT NOT NULL,
    task TEXT NOT NULL,
    summary TEXT NOT NULL
);