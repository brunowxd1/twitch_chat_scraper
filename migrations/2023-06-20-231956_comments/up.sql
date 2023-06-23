-- Your SQL goes here
CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    user_id SERIAL REFERENCES users(id),
    comment VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL
);