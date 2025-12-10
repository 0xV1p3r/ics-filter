CREATE TYPE status_enum AS ENUM ('archived', 'deleted', 'trash');
CREATE TYPE filter_type_enum AS ENUM ('blacklist', 'whitelist');
CREATE TYPE filter_criteria_type_enum AS ENUM ('description', 'description_contains', 'location', 'location_contains', 'summary', 'summary_contains');

CREATE TABLE IF NOT EXISTS filter (
    id SERIAL,
    name TEXT NOT NULL,
    filter_type filter_type_enum NOT NULL,
    status status_enum,

    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT pk_filter PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS filter_criteria (
    id SERIAL,
    filter_id INTEGER,
    filter_criteria_type filter_criteria_type_enum NOT NULL,
    value TEXT NOT NULL,

    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT pk_filter_criteria PRIMARY KEY (id),
    CONSTRAINT fk_filter_criteria_filter FOREIGN KEY (filter_id) REFERENCES filter(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS calendar (
    id SERIAL,
    name TEXT NOT NULL,
    color TEXT NOT NULL,

    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT pk_calendar PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS remote_calendar (
    id INTEGER,
    url TEXT,
    last_refresh TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT pk_remote_calendar PRIMARY KEY (id),
    CONSTRAINT fk_remote_calendar_calendar FOREIGN KEY (id) REFERENCES calendar(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS merged_calendar (
    id INTEGER,
    source_id INTEGER,

    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT pk_merged_calendar PRIMARY KEY (id),
    CONSTRAINT fk_merged_calendar_calendar FOREIGN KEY (id) REFERENCES calendar(id),
    CONSTRAINT fk_merged_calendar_calendar_source FOREIGN KEY (source_id) REFERENCES calendar(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS filtered_calendar (
    id INTEGER,
    source_id INTEGER,
    filter_id INTEGER,

    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT pk_filtered_calendar PRIMARY KEY (id),
    CONSTRAINT fk_filtered_calendar_calendar FOREIGN KEY (id) REFERENCES calendar(id) ON DELETE CASCADE,
    CONSTRAINT fk_filtered_calendar_calendar_source FOREIGN KEY (source_id) REFERENCES calendar(id) ON DELETE CASCADE,
    CONSTRAINT fk_filtered_calendar_filter FOREIGN KEY (filter_id) REFERENCES filter(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS event (
    id SERIAL,
    calendar_id INTEGER,
    status status_enum,

    summary TEXT NOT NULL,
    location TEXT NOT NULL,
    description TEXT,
    start_date_time TIMESTAMP WITH TIME ZONE NOT NULL,
    end_date_time TIMESTAMP WITH TIME ZONE NOT NULL,

    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT pk_event PRIMARY KEY (id),
    CONSTRAINT fk_event_calendar FOREIGN KEY (calendar_id) REFERENCES calendar(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS event_snapshot (
    id INTEGER,
    snapshot_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    calendar_id INTEGER,
    status status_enum,

    summary TEXT NOT NULL,
    location TEXT NOT NULL,
    description TEXT,
    start_date_time TIMESTAMP WITH TIME ZONE NOT NULL,
    end_date_time TIMESTAMP WITH TIME ZONE NOT NULL,

    CONSTRAINT pk_event_snapshot PRIMARY KEY (id, snapshot_at),
    CONSTRAINT fk_event_snapshot_event FOREIGN KEY (id) REFERENCES event(id) ON DELETE CASCADE,
    CONSTRAINT fk_event_snapshot_calendar FOREIGN KEY (calendar_id) REFERENCES calendar(id) ON DELETE CASCADE
);
