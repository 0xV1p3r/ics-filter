// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "filter_criteria_type_enum"))]
    pub struct FilterCriteriaTypeEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "filter_type_enum"))]
    pub struct FilterTypeEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "status_enum"))]
    pub struct StatusEnum;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::StatusEnum;

    calendar (id) {
        id -> Int4,
        color -> Text,
        name -> Text,
        status -> Nullable<StatusEnum>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::StatusEnum;

    event (id) {
        id -> Int4,
        calendar_id -> Nullable<Int4>,
        status -> Nullable<StatusEnum>,
        summary -> Text,
        location -> Text,
        description -> Nullable<Text>,
        start_date_time -> Timestamptz,
        end_date_time -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::StatusEnum;

    event_snapshot (id, snapshot_at) {
        id -> Int4,
        snapshot_at -> Timestamptz,
        calendar_id -> Nullable<Int4>,
        status -> Nullable<StatusEnum>,
        summary -> Text,
        location -> Text,
        description -> Nullable<Text>,
        start_date_time -> Timestamptz,
        end_date_time -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::FilterTypeEnum;
    use super::sql_types::StatusEnum;

    filter (id) {
        id -> Int4,
        name -> Text,
        filter_type -> FilterTypeEnum,
        status -> Nullable<StatusEnum>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::FilterCriteriaTypeEnum;

    filter_criteria (id) {
        id -> Int4,
        filter_id -> Nullable<Int4>,
        filter_criteria_type -> FilterCriteriaTypeEnum,
        value -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    filtered_calendar (id) {
        id -> Int4,
        source_id -> Nullable<Int4>,
        filter_id -> Nullable<Int4>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    merged_calendar (id) {
        id -> Int4,
        source_id -> Nullable<Int4>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    remote_calendar (id) {
        id -> Int4,
        url -> Nullable<Text>,
        last_refresh -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(event -> calendar (calendar_id));
diesel::joinable!(event_snapshot -> calendar (calendar_id));
diesel::joinable!(event_snapshot -> event (id));
diesel::joinable!(filter_criteria -> filter (filter_id));
diesel::joinable!(filtered_calendar -> filter (filter_id));
diesel::joinable!(remote_calendar -> calendar (id));

diesel::allow_tables_to_appear_in_same_query!(
    calendar,
    event,
    event_snapshot,
    filter,
    filter_criteria,
    filtered_calendar,
    merged_calendar,
    remote_calendar,
);
