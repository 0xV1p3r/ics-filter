CREATE TABLE events (
    id SERIAL PRIMARY KEY,
    calendar_id INTEGER NOT NULL,

    uid TEXT NOT NULL,
    summary TEXT NOT NULL,
    location TEXT NOT NULL,
    description TEXT,

    start_date TIMESTAMP NOT NULL,
    end_date TIMESTAMP NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (calendar_id) REFERENCES calendars(id) ON DELETE CASCADE
);
