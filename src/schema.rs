// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "filter_type_enum"))]
    pub struct FilterTypeEnum;
}

diesel::table! {
    calendars (id) {
        id -> Int4,
        name -> Text,
        url -> Text,
        color -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    event_snapshots (id) {
        id -> Int4,
        event_id -> Int4,
        calendar_id -> Int4,
        uid -> Text,
        summary -> Text,
        location -> Text,
        description -> Nullable<Text>,
        start_date -> Timestamp,
        end_date -> Timestamp,
        timestamp -> Timestamp,
    }
}

diesel::table! {
    events (id) {
        id -> Int4,
        calendar_id -> Int4,
        uid -> Text,
        summary -> Text,
        location -> Text,
        description -> Nullable<Text>,
        start_date -> Timestamp,
        end_date -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    filter_criteria (id) {
        id -> Int4,
        filter_id -> Int4,
        criteria_type -> Text,
        value -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    filtered_calendars (id) {
        id -> Int4,
        filter_id -> Int4,
        calendar_id -> Int4,
        name -> Text,
        color -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::FilterTypeEnum;

    filters (id) {
        id -> Int4,
        filter_type -> FilterTypeEnum,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
