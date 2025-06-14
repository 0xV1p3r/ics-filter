# ICS Filter

ICS Filter is a tool designed to manage and filter events from ICS calendar files. It allows you to synchronize and clean your calendars based on a configurable blacklist, deployable via Docker, and offers notification and Git archiving functionalities.

## Features

*   **Event Filtering**: Takes a URL to an ICS file or calendar and filters out events based on a configured blacklist.
*   **Docker Deployment**: Easily deployable and manageable via Docker.
*   **Notification System**:
    *   **Gotify**: Send notifications about removed, added, or modified events via Gotify.
    *   **Email**: Send email notifications for calendar changes.
*   **Git Archiving**: Optionally commit all modifications of tracked ICS files to a Git repository, providing a historical record of changes.

## Configuration

The `config.toml` file is used to configure ICS Filter. Below is an example of the configuration structure:

```toml
# Example config.toml

[[calendars]]
url = "https://example.com/my-calendar.ics" # URL to your ICS calendar file
blacklist = ["Meeting with John", "Dentist Appointment"] # List of event summaries to filter out

# Git Archiving Configuration (Optional - Comment out/remove if not used)
[git.signature]
username = "Your Git Username" # Username for Git commits
email = "your.email@example.com" # Email for Git commits

[git.remote]
domain = "github.com" # Domain of your Git repository (e.g., github.com, gitlab.com)
repository = "your-repo" # Name of your Git repository 
username = "your-git-username" # Username for accessing the Git repository
token = "your-personal-access-token" # Personal Access Token for Git authentication

# Notification Configuration (Optional - Comment out/remove if not used)

[notifications.email]
smtp_server = "smtp.example.com:587" # SMTP server address and port
username = "your-email@example.com" # Email account username
password = "your-email-password" # Email account password
recipients = ["recipient1@example.com", "recipient2@example.com"] # List of recipient email addresses

[notifications.gotify]
server = "https://gotify.example.com" # Gotify server URL
token = "your-gotify-app-token" # Gotify application token
```

## Deployment with Docker

ICS Filter is designed to be easily deployed using Docker.

### `docker-compose.yaml` Example

```yaml
services:
  app:
    image: ghcr.io/0xv1p3r/ics-filter:latest
    restart: unless-stopped
    ports:
      - 5000:80
    volumes:
      - ./config.toml:/app/config.toml:ro
```
