CREATE OR REPLACE FUNCTION fn_event_snapshot_before_update() RETURNS trigger AS $$
BEGIN
    IF NEW IS DISTINCT FROM OLD THEN
        INSERT INTO event_snapshot (id, calendar_id, status, summary, location, description, start_date_time, end_date_time)
        VALUES (OLD.id, OLD.calendar_id, OLD.status, OLD.summary, OLD.location, OLD.description, OLD.start_date_time, OLD.end_date_time);
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER trg_event_snapshot BEFORE UPDATE ON event
FOR EACH ROW EXECUTE FUNCTION fn_event_snapshot_before_update();

CREATE OR REPLACE FUNCTION fn_set_updated_at() RETURNS trigger AS $$
BEGIN
    IF NEW IS DISTINCT FROM OLD THEN
        NEW.updated_at := CURRENT_TIMESTAMP;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER trg_set_updated_at_calendar BEFORE UPDATE ON calendar
FOR EACH ROW EXECUTE FUNCTION fn_set_updated_at();

CREATE OR REPLACE TRIGGER trg_set_updated_at_filtered_calendar BEFORE UPDATE ON filtered_calendar
FOR EACH ROW EXECUTE FUNCTION fn_set_updated_at();

CREATE OR REPLACE TRIGGER trg_set_updated_at_merged_calendar BEFORE UPDATE ON merged_calendar
FOR EACH ROW EXECUTE FUNCTION fn_set_updated_at();

CREATE OR REPLACE TRIGGER trg_set_updated_at_remote_calendar BEFORE UPDATE ON remote_calendar
FOR EACH ROW EXECUTE FUNCTION fn_set_updated_at();

CREATE OR REPLACE TRIGGER trg_set_updated_at_filter BEFORE UPDATE ON filter
FOR EACH ROW EXECUTE FUNCTION fn_set_updated_at();

CREATE OR REPLACE TRIGGER trg_set_updated_at_filter_criteria BEFORE UPDATE ON filter_criteria
FOR EACH ROW EXECUTE FUNCTION fn_set_updated_at();
