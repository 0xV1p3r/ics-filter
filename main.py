import configparser
import datetime
import json
import os

import requests
from ics import Calendar, Event

from constants import (BLACKLIST_FILE, CONFIG_FILE, ICS_FILE_LOCATION,
                       SEPARATOR_LENGTH, URL_FILE, GIT_USER_ENV, GIT_PASSWORD_ENV)
from watchdog import check_for_change, get_modified_attributes, run_watchdog
from version_control import check_for_repo, setup_repo, sync_repo


def now():
    return f"{datetime.datetime.now().strftime('%Y-%m-%d %H:%M:%S')}"

def fetch():
    with open(URL_FILE, "r") as f:
        urls = json.loads(f.read())
    data_sets = {}
    for url in urls:
        print(f"[{now()}] Fetching '{url}'...")
        fetched_ics_data = requests.get(url).text
        filename = url.rsplit("/", 1)[-1]
        data_sets[filename] = fetched_ics_data
    return data_sets

def get_current_data(filename):
    #TODO: Implement checks (path exists, etc.)
    with open(f"{ICS_FILE_LOCATION}/{filename}", "r") as f:
        return f.read()

def filter_calendar(calendar: Calendar) -> Calendar:
    with open(BLACKLIST_FILE, "r") as f:
        blacklist = json.loads(f.read())
    input_calendar = calendar
    output_calendar = Calendar()

    for event in list(input_calendar.timeline):
        if event.description.split("\n")[0] not in blacklist:
            output_calendar.events.add(event)

    return output_calendar

def build(build_data):

    print(f"[{now()}] Building {len(build_data)} new ics files...")

    for calendar in build_data:
        filename = calendar["filename"]
        input_calendar = Calendar(calendar["data"])
        output_calendar = filter_calendar(input_calendar)

        with open(f"{ICS_FILE_LOCATION}/{filename}", "w") as f:
            f.writelines(output_calendar.serialize_iter())

    print(f"[{now()}] Finished Build.")

def format_event(event: Event):
    return f"""{SEPARATOR_LENGTH * "="}
Event: {event.name}
{SEPARATOR_LENGTH * "-"}
{event.description}
{SEPARATOR_LENGTH * "-"}
Location: {event.location}
{SEPARATOR_LENGTH * "-"}
Start: {event.begin}
End: {event.end}
{SEPARATOR_LENGTH * "="}
"""

def open_config():
    config = configparser.ConfigParser()
    config.read(CONFIG_FILE)
    return config

def send_notification(message: str, user_token, app_token):
    print(f"[{now()}] Sending message via Pushover...")
    try:
        requests.post(url="https://api.pushover.net/1/messages.json",
                      data={"token": app_token, "user": user_token, "message": message},
                      )
        print(f"[{now()}] Message sent.")
    except requests.exceptions.RequestException as e:
        print(f"[{now()}]An error occurred while sending a message via Pushover: {str(e)}")

def dispatch_reports(config, reports):
    for report_data in reports:
        report = report_data["data"]
        calendar = report_data["filename"]
        if report["added"]:
            for event in report["added"]:
                message = f"{calendar} -- Event added:\n{format_event(event)}"
                send_notification(
                    message=message,
                    user_token=config["PUSHOVER"]["token"],
                    app_token=config["PUSHOVER"]["app_token"]
                )
        if report["removed"]:
            for event in report["removed"]:
                message = f"{calendar} -- Event removed:\n{format_event(event)}"
                send_notification(
                    message=message,
                    user_token=config["PUSHOVER"]["token"],
                    app_token=config["PUSHOVER"]["app_token"]
                )
        if report["modified"]:
            for event_pair in report["modified"]:
                modified_fields = get_modified_attributes(event_pair[0], event_pair[1])
                message = f"{calendar} -- Event modified:\n{event_pair[0].name}\n"
                for field in modified_fields:
                    message += f"{field[0]}: {field[1]} -> {field[2]}\n"
                send_notification(
                    message=message,
                    user_token=config["PUSHOVER"]["token"],
                    app_token=config["PUSHOVER"]["app_token"]
                )

def main():
    data_sets = fetch()
    to_build = []
    to_sync = []
    reports = []
    for filename in list(data_sets.keys()):

        data_set = data_sets[filename]
        new_data = filter_calendar(Calendar(data_set))
        old_data = get_current_data(filename)

        if check_for_change(old_data, new_data.serialize()):
            print(f"[{now()}] Change detected. Generating report...")
            to_build.append({"filename": filename, "data": new_data.serialize()})
            report = run_watchdog(Calendar(old_data), new_data)
            reports.append({"filename": filename, "data": report})
            to_sync.append(filename)
            print(f"[{now()}] Finished report. (Insertions: {len(report['added'])}, Deletions: {len(report['removed'])}, Modifications: {len(report['modified'])}).")
        else:
            print(f"[{now()}] No changes detected.")

    build(to_build)
    config = open_config()
    if config["PUSHOVER"]["enabled"].lower() == "true":
        dispatch_reports(config, reports)
    if config["GIT"]["enabled"].lower() == "true":
        if not check_for_repo():
            setup_repo(
                remote_name=config["GIT"]["remote_name"],
                domain=config["GIT"]["remote_domain"],
                username=os.getenv(GIT_USER_ENV),
                password=os.getenv(GIT_PASSWORD_ENV)
            )
        sync_repo(to_sync)

if __name__ == "__main__":
    main()
