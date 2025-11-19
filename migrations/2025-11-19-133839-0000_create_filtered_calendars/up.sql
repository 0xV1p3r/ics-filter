CREATE TABLE filtered_calendars (
    filter_id INTEGER NOT NULL,
    calendar_id INTEGER NOT NULL,

    name TEXT NOT NULL,
    color TEXT NOT NULL,

    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,

    PRIMARY KEY (filter_id, calendar_id),
    FOREIGN KEY (filter_id) REFERENCES filters(id) ON DELETE CASCADE,
    FOREIGN KEY (calendar_id) REFERENCES calendars(id) ON DELETE CASCADE
);
