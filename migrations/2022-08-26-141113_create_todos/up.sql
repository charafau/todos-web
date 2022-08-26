-- Your SQL goes here
CREATE TABLE todos (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    is_done BOOLEAN NOT NULL DEFAULT 'f'
)