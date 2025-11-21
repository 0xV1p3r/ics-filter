CREATE TABLE event_snapshots (
    id SERIAL PRIMARY KEY,
    event_id INTEGER NOT NULL,
    calendar_id INTEGER NOT NULL,

    uid TEXT NOT NULL,
    summary TEXT NOT NULL,
    location TEXT NOT NULL,
    description TEXT,
    start_date TIMESTAMP NOT NULL,
    end_date TIMESTAMP NOT NULL,

    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE CASCADE,
    FOREIGN KEY (calendar_id) REFERENCES calendars(id) ON DELETE CASCADE
);
