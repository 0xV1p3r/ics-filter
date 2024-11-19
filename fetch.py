import datetime
import hashlib
import json

import requests


def now():
    return f"{datetime.datetime.now().strftime('%Y-%m-%d %H:%M:%S')}"

with open("urls.json", "r") as f:
    urls = json.loads(f.read())

with open("hash_cache.json", "r") as f:
    hash_cache = json.loads(f.read())

to_build = []
modified_hash_cache = False
for url in urls:
    print(f"[{now()}] Fetching '{url}'...")
    fetched_ics_data = requests.get(url).text
    hash_from_data = hashlib.sha256(fetched_ics_data.encode("utf-8")).hexdigest()
    filename = url.rsplit("/", 1)[-1]
    if filename not in list(hash_cache.keys()):
        print(f"[{now()}] Adding new hash")
        hash_cache[filename] = hash_from_data
        to_build.append({"filename": filename, "data": fetched_ics_data})
        modified_hash_cache = True
        continue

    stored_hash = hash_cache[filename]
   
    if stored_hash != hash_from_data:
        print(f"[{now()}] File changed! old: '{stored_hash}' -- new: '{hash_from_data}'")
        to_build.append({"filename": filename, "data": fetched_ics_data})
        modified_hash_cache = True
        hash_cache[filename] = hash_from_data
    else:
        print(f"[{now()}] No change detected.")

with open("build_data.json", "w") as f:
    f.write(json.dumps(to_build))

if modified_hash_cache:
    with open("hash_cache.json", "w") as f:
        f.write(json.dumps(hash_cache, indent=4))
