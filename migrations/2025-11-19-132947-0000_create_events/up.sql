PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS events (
    id INTEGER NOT NULL PRIMARY KEY,
    calendar_id INTEGER NOT NULL,

    uid TEXT NOT NULL,
    summary TEXT NOT NULL,
    location TEXT NOT NULL,
    description TEXT,

    timezone TEXT NOT NULL,
    start_date DATETIME NOT NULL,
    end_date DATETIME NOT NULL,

    FOREIGN KEY (calendar_id) REFERENCES calendars(id) ON DELETE CASCADE,
    UNIQUE (calendar_id, uid)
);
