// @generated automatically by Diesel CLI.

diesel::table! {
    calendars (id) {
        id -> Integer,
        name -> Text,
        url -> Text,
        color -> Text,
    }
}

diesel::table! {
    event_snapshots (id) {
        id -> Integer,
        event_id -> Integer,
        calendar_id -> Integer,
        timestamp -> Timestamp,
        uid -> Text,
        summary -> Text,
        location -> Text,
        description -> Nullable<Text>,
        timezone -> Text,
        start_date -> Timestamp,
        end_date -> Timestamp,
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
        timezone -> Text,
        start_date -> Timestamp,
        end_date -> Timestamp,
    }
}

diesel::table! {
    filter_criteria (id) {
        id -> Integer,
        criteria_type -> Text,
        value -> Text,
    }
}

diesel::table! {
    filter_criteria_filters (filter_id, filter_criteria_id) {
        filter_id -> Integer,
        filter_criteria_id -> Integer,
    }
}

diesel::table! {
    filtered_calendars (filter_id, calendar_id) {
        filter_id -> Integer,
        calendar_id -> Integer,
        name -> Text,
        color -> Text,
    }
}

diesel::table! {
    filters (id) {
        id -> Integer,
        filter_type -> Text,
        name -> Text,
    }
}

diesel::joinable!(event_snapshots -> calendars (calendar_id));
diesel::joinable!(event_snapshots -> events (event_id));
diesel::joinable!(events -> calendars (calendar_id));
diesel::joinable!(filter_criteria_filters -> filter_criteria (filter_criteria_id));
diesel::joinable!(filter_criteria_filters -> filters (filter_id));
diesel::joinable!(filtered_calendars -> calendars (calendar_id));
diesel::joinable!(filtered_calendars -> filters (filter_id));

diesel::allow_tables_to_appear_in_same_query!(
    calendars,
    event_snapshots,
    events,
    filter_criteria,
    filter_criteria_filters,
    filtered_calendars,
    filters,
);
