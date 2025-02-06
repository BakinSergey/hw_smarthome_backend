-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS rooms
(
    id      uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name    varchar not null,
    home_id uuid    not null references homes ON DELETE CASCADE,
    UNIQUE (name, home_id)
)