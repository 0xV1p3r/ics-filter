import datetime
import os
from git import Repo

from constants import ICS_FILE_LOCATION, REPO_LOCATION


def now():
    return f"{datetime.datetime.now().strftime('%Y-%m-%d %H:%M:%S')}"

def check_for_repo():
    if not os.path.exists(f"{REPO_LOCATION}/.git"):
        return False
    return True

def setup_repo(remote_name, domain, username, password):
    remote = f"https://{password}@{domain}/{username}/{remote_name}.git"
    repo = Repo.clone_from(remote, REPO_LOCATION)

def sync_repo(filenames):
    repo = Repo(REPO_LOCATION)
    origin = repo.remote(name="origin")
    for filename in filenames:

        with open(f"{ICS_FILE_LOCATION}/{filename}", "r") as f:
            new_data = f.read()
        with open(f"{REPO_LOCATION}/{filename}", "w") as f:
            f.write(new_data)

        repo.git.add(filename)
        repo.index.commit(f"{now()} -- {filename}")
        origin.push()
