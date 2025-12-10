DROP TRIGGER IF EXISTS trg_set_updated_at_filter_criteria ON filter_criteria;
DROP TRIGGER IF EXISTS trg_set_updated_at_filter ON filter;
DROP TRIGGER IF EXISTS trg_set_updated_at_remote_calendar ON remote_calendar;
DROP TRIGGER IF EXISTS trg_set_updated_at_merged_calendar ON merged_calendar;
DROP TRIGGER IF EXISTS trg_set_updated_at_filtered_calendar ON filtered_calendar;
DROP TRIGGER IF EXISTS trg_set_updated_at_calendar ON calendar;

DROP TRIGGER IF EXISTS trg_event_snapshot ON event;

DROP FUNCTION IF EXISTS fn_set_updated_at() CASCADE;
DROP FUNCTION IF EXISTS fn_event_snapshot_before_update() CASCADE;
