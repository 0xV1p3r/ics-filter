// @generated automatically by Diesel CLI.

diesel::table! {
    calendars (id) {
        id -> Integer,
        name -> Text,
        url -> Text,
        color -> Text,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
    }
}

diesel::table! {
    event_snapshots (id) {
        id -> Integer,
        event_id -> Integer,
        calendar_id -> Integer,
        uid -> Text,
        summary -> Text,
        location -> Text,
        description -> Nullable<Text>,
        start_date -> TimestamptzSqlite,
        end_date -> TimestamptzSqlite,
        timestamp -> TimestamptzSqlite,
    }
}

diesel::table! {
    events (id) {
        id -> Integer,
        calendar_id -> Integer,
        uid -> Text,
        summary -> Text,
        location -> Text,
        description -> Nullable<Text>,
        start_date -> TimestamptzSqlite,
        end_date -> TimestamptzSqlite,
        created_at -> TimestamptzSqlite,
    }
}

diesel::table! {
    filter_criteria (id) {
        id -> Integer,
        filter_id -> Integer,
        criteria_type -> Text,
        value -> Text,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
    }
}

diesel::table! {
    filtered_calendars (id) {
        id -> Integer,
        filter_id -> Integer,
        calendar_id -> Integer,
        name -> Text,
        color -> Text,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
    }
}

diesel::table! {
    filters (id) {
        id -> Integer,
        filter_type -> Text,
        name -> Text,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
    }
}

diesel::joinable!(event_snapshots -> calendars (calendar_id));
diesel::joinable!(event_snapshots -> events (event_id));
diesel::joinable!(events -> calendars (calendar_id));
diesel::joinable!(filter_criteria -> filters (filter_id));
diesel::joinable!(filtered_calendars -> calendars (calendar_id));
diesel::joinable!(filtered_calendars -> filters (filter_id));

diesel::allow_tables_to_appear_in_same_query!(
    calendars,
    event_snapshots,
    events,
    filter_criteria,
    filtered_calendars,
    filters,
);
