-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS devices
(
    id      uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name    varchar not null,
    data    json    not null,
    room_id uuid    not null references rooms ON DELETE CASCADE,
    UNIQUE (name, room_id)
)