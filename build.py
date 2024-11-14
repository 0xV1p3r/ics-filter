import datetime
import json
import os

from ics import Calendar


def now():
    return f"{datetime.datetime.now().strftime('%Y-%m-%d %H:%M:%S')}"

if not os.path.exists("build_data.json"):
    print(f"[{now()}] Nothind to build! Exiting...")
    exit()

with open("blacklist.json", "r") as f:
    blacklist = json.loads(f.read())

with open("build_data.json", "r") as f:
    build_data = json.loads(f.read())

print(f"[{now()}] Building {len(build_data)} new ics files...")
for calendar in build_data:
    filename = calendar["filename"]
    input_calendar = Calendar(calendar["data"])
    output_calendar = Calendar()

    for event in list(input_calendar.timeline):
        if event.description.split("\n")[0] not in blacklist:
            output_calendar.events.add(event)

    with open(f"./ics_files/{filename}", "w") as f:
        f.writelines(output_calendar.serialize_iter())

os.remove("build_data.json")
print(f"[{now()}] Finished Build.")
