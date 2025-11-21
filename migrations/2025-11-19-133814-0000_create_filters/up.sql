CREATE TYPE filter_type_enum AS ENUM ('blacklist', 'whitelist');
CREATE TABLE filters (
    id SERIAL PRIMARY KEY,
    filter_type filter_type_enum NOT NULL,
    name TEXT NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
