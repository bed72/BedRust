-- Your SQL goes here
CREATE TABLE coffees (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(32) NOT NULL,
    price FLOAT NOT NULL,
    CONSTRAINT unique_name UNIQUE (name)
);