-- Your SQL goes here
CREATE TABLE coffees (
    id UUID DEFAULT gen_random_uuid(),
    name VARCHAR(32) NOT NULL,
    price FLOAT NOT NULL,
    created_at TIMESTAMP DEFAULT now(),
    updated_at TIMESTAMP DEFAULT now(),
    PRIMARY KEY (id),
    CONSTRAINT unique_name UNIQUE (name)
);