CREATE TABLE filter_criteria_filters (
    filter_id INTEGER NOT NULL,
    filter_criteria_id INTEGER NOT NULL,

    PRIMARY KEY (filter_id, filter_criteria_id),
    FOREIGN KEY (filter_id) REFERENCES filters(id) ON DELETE CASCADE,
    FOREIGN KEY (filter_criteria_id) REFERENCES filter_criteria(id) ON DELETE CASCADE
)
