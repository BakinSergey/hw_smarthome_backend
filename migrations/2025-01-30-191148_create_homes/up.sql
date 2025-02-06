-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS homes
(
    id   uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name varchar not null UNIQUE
)