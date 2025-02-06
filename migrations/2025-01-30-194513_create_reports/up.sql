-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS reports
(
    id      uuid PRIMARY KEY     DEFAULT uuid_generate_v4(),
    home_id uuid        not null references homes ON DELETE CASCADE,
    query   json        not null,
    content json        not null,
    created timestamptz not null DEFAULT now()
)