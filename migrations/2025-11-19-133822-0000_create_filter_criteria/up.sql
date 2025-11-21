CREATE TYPE criteria_type_enum AS ENUM ('description', 'location', 'summary');
CREATE TABLE filter_criteria (
    id SERIAL PRIMARY KEY,
    filter_id INTEGER NOT NULL,

    criteria_type TEXT NOT NULL,
    value TEXT NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (filter_id) REFERENCES filters(id) ON DELETE CASCADE
);
