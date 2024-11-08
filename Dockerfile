FROM python:3.12-slim

# Install uv.
COPY --from=ghcr.io/astral-sh/uv:latest /uv /uvx /bin/

# Install cron
RUN apt-get update && apt-get -y install cron

# Copy the application into the container.
COPY ./api.py /app/api.py
COPY ./build.py /app/build.py
COPY ./fetch.py /app/fetch.py
COPY ./pyproject.toml /app/pyproject.toml
COPY ./uv.lock /app/uv.lock

# Install the application dependencies.
WORKDIR /app
RUN uv sync --frozen --no-cache

# Add cron job
COPY ./build-and-fetch-cron /etc/cron.d/build-and-fetch-cron
RUN chmod 0644 /etc/cron.d/build-and-fetch-cron
RUN crontab /etc/cron.d/build-and-fetch-cron

# Run the application.
CMD ["/app/.venv/bin/fastapi", "run", "api.py", "--port", "80", "--host", "0.0.0.0"]
