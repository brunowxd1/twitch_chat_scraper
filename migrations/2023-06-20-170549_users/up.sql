-- Your SQL goes here
CREATE TABLE users(
    id         SERIAL PRIMARY KEY,
    username   VARCHAR NOT NULL,
    is_sub      BOOLEAN NOT NULL,
    is_partner BOOLEAN NOT NULL,
    is_mod      BOOLEAN NOT NULL,
    is_vip      BOOLEAN NOT NULL,
    is_admin    BOOLEAN NOT NULL,
    is_broadcaster BOOLEAN NOT NULL
);