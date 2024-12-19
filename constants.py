REPO_NAME = "git_repo"
SETUP_CREDENTIAL_HELPER = "git config credential.helper '!f() { sleep 1; echo \"username=${GIT_USER}\"; echo \"password=${GIT_PASSWORD}\"; }; f'"
URL_FILE = "urls.json"
ICS_FILE_LOCATION = "ics_files"
BLACKLIST_FILE = "blacklist.json"
CONFIG_FILE = "config.ini"
SEPARATOR_LENGTH = 30
BLACKLIST_EVENT_FIELD = ["created"]
DIFF_NEW = 'diff --unchanged-line-format="" --old-line-format="" --new-line-format=":%dn: %L"'
DIFF_DEL = 'diff --unchanged-line-format="" --old-line-format=":%dn: %L" --new-line-format=""'
OD_FILE = "/tmp/old_data.ics"
ND_FILE = "/tmp/new_data.ics"
