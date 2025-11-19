CREATE TABLE event_snapshots (
    id INTEGER NOT NULL PRIMARY KEY,
    event_id INTEGER NOT NULL,
    calendar_id INTEGER NOT NULL,

    uid TEXT NOT NULL,
    summary TEXT NOT NULL,
    location TEXT NOT NULL,
    description TEXT,
    timezone TEXT NOT NULL,
    start_date DATETIME NOT NULL,
    end_date DATETIME NOT NULL,

    created_at DATETIME NOT NULL,
    timestamp DATETIME NOT NULL,

    FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE CASCADE,
    FOREIGN KEY (calendar_id) REFERENCES calendars(id) ON DELETE CASCADE
);
