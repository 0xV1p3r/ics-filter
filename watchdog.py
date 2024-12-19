import inspect
import os
import shlex
import subprocess
from typing import Any

from ics import Calendar, Event

from constants import (BLACKLIST_EVENT_FIELD, DIFF_DEL, DIFF_NEW, ND_FILE,
                       OD_FILE)


def execute_command(command):

    process = subprocess.Popen(shlex.split(command), stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                               universal_newlines=True)
    stdout, stderr = process.communicate()
    return_code = process.poll()

    return stdout, stderr, return_code

def cleanup_cal_data(data: str) -> str:
    lines = data.splitlines()
    for line in lines:
        if "DTSTAMP" in line:
            lines.remove(line)
    return "".join(lines)

def check_for_change(old_data, new_data) -> bool:

    old_data = cleanup_cal_data(old_data)
    new_data = cleanup_cal_data(new_data)

    with open(OD_FILE, "w") as old_ics:
        old_ics.write(old_data)
    with open(ND_FILE, "w") as new_ics:
        new_ics.write(new_data)

    files = f" {OD_FILE} {ND_FILE}"
    stdout, stderr, return_code = execute_command(DIFF_NEW + files)
    if stdout:
        os.remove(OD_FILE)
        os.remove(ND_FILE)
        return True

    stdout, stderr, return_code = execute_command(DIFF_DEL + files)
    if stdout:
        os.remove(OD_FILE)
        os.remove(ND_FILE)
        return True

    os.remove(OD_FILE)
    os.remove(ND_FILE)
    return False

def get_uids(cal: Calendar) -> list[str]:
    uids = []
    for idx, event in enumerate(list(cal.timeline)):
        uids.append(event.uid)
    return uids

def compare_uids(uids_old: list[str], uids_new: list[str]) -> tuple[bool, bool, tuple[set, set]]:
    delta_added = set(uids_new) - set(uids_old)
    delta_removed = set(uids_old) - set(uids_new)
    event_added = True if delta_added else False
    event_removed = True if delta_removed else False
    event_data = (delta_added, delta_removed)
    return event_added, event_removed, event_data

def get_event_by_uid(uid: str, calendar: Calendar) -> Event|None:
    for event in calendar.events:
        if event.uid == uid:
            return event
    return None

def compare_calendar_event_modifications(old_cal: Calendar, new_cal: Calendar) -> list[tuple[Event, Event]]:
    uids = set(get_uids(old_cal)) & set(get_uids(new_cal))
    modified_events = []
    for uid in uids:
        event1 = get_event_by_uid(uid, old_cal)
        event2 = get_event_by_uid(uid, new_cal)

        if unpack_event(event1) != unpack_event(event2):
            modified_events.append((event1, event2))

    return modified_events

def unpack_event(event: Event) -> dict:
    event_fields = {}
    for member in inspect.getmembers(event):
        if not member[0].startswith("_"):
            if not inspect.ismethod(member[1]):
                if member[0] not in BLACKLIST_EVENT_FIELD:
                    event_fields[member[0]] = member[1]
    return event_fields

def get_modified_attributes(event1: Event, event2: Event) -> list[tuple[str, str, str]]:
    modified_attributes = []
    event1_data = unpack_event(event1)
    event2_data = unpack_event(event2)
    for field, value in event1_data.items():
        value2 = event2_data.get(field)
        if value != value2:
            modified_attributes.append((field, value, value2))

    return modified_attributes

def run_watchdog(old_cal: Calendar, new_cal: Calendar) -> dict[str, list[Any] | list[tuple[Event, Event]]]:
    report = {
        "added": [],
        "removed": [],
        "modified": []
    }
    event_added, event_removed, event_data = compare_uids(get_uids(old_cal), get_uids(new_cal))
    if event_added:
        new_event_uids = event_data[0]
        for new_event_uid in new_event_uids:
            event = get_event_by_uid(new_event_uid, new_cal)
            report["added"].append(event)
    if event_removed:
        new_event_uids = event_data[1]
        for new_event_uid in new_event_uids:
            event = get_event_by_uid(new_event_uid, old_cal)
            report["removed"].append(event)

    modified_events = compare_calendar_event_modifications(old_cal, new_cal)
    if modified_events:
        report["modified"] = modified_events

    return report
