import datetime
import os

from constants import ICS_FILE_LOCATION, REPO_NAME, SETUP_CREDENTIAL_HELPER
from watchdog import execute_command


def now():
    return f"{datetime.datetime.now().strftime('%Y-%m-%d %H:%M:%S')}"

def check_for_repo():
    if not os.path.exists(f"{REPO_NAME}/.git"):
        return False
    return True

def setup(url):
    print(f"[{now()}] Setting up git repository...")

    if not check_for_repo():
        if not os.path.exists(REPO_NAME):
            os.mkdir(REPO_NAME)

        os.chdir(REPO_NAME)
        print(f"[{now()}] Initializing git repository...", end="")
        stdout, stderr, status = execute_command("git init")
        print(" done.")
        print(stdout, stderr)
        print(f"[{now()}] Setting up credential helper...", end="")
        stdout, stderr, status = execute_command(SETUP_CREDENTIAL_HELPER)
        print(" done.")
        print(stdout, stderr)
        print(f"[{now()}] Adding remote to git repository...", end="")
        stdout, stderr, status = execute_command(f"git remote add origin {url}")
        print(" done.")
        print(stdout, stderr)
        print(f"[{now()}] Pulling from remote...", end="")
        stdout, stderr, status = execute_command("git pull origin main")
        print(" done.")
        print(stdout, stderr)
        stdout, stderr, status = execute_command("git branch --set-upstream-to=origin/main main")
        print(stdout, stderr)
        os.chdir("..")
        print(f"[{now()}] Finished setting up git repository.")
    else:
        print(f"[{now()}] Aborting! git repository already exists.")

def save_changes(filenames):
    for filename in filenames:
        print(f"[{now()}] Syncing '{filename}'...", end="")
        with open(f"{ICS_FILE_LOCATION}/{filename}", "r") as f:
            new_data = f.read()
        with open(f"{REPO_NAME}/{filename}", "w") as f:
            f.write(new_data)

        os.chdir(REPO_NAME)
        stdout, stderr, status = execute_command(f"git add {filename}")
        print(stdout, stderr)
        stdout, stderr, status = execute_command(f"git commit -m '{now()} -- {filename}'")
        print(stdout, stderr)
        stdout, stderr, status = execute_command(f"git push origin {filename}")
        print(stdout, stderr)
        os.chdir("..")
        print(" done.")
