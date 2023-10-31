-- Add up migration script here
-- Add migration script here
CREATE TABLE coffees (
    id UUID DEFAULT gen_random_uuid(),
    name VARCHAR(32) NOT NULL,
    price FLOAT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    PRIMARY KEY (id),
    CONSTRAINT unique_name UNIQUE (name)
);
