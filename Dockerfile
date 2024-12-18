FROM python:3.12-slim

# Install cron
RUN apt-get update && apt-get -y install cron

# Copy the application into the container.
COPY ./api.py /app/api.py
COPY ./main.py /app/main.py
COPY ./watchdog.py /app/watchdog.py
COPY ./requirements.txt /app/requirements.txt

# Install the application dependencies.
WORKDIR /app
RUN pip3 install -r requirements.txt

# Add cron job
RUN touch /app/crontab.log
RUN crontab -l | { cat; echo "30 * * * * cd /app && /usr/local/bin/python3 /app/main.py >> /app/crontab.log 2>&1"; } | crontab -

# Run the application.
CMD cron -f & /usr/local/bin/python3 /app/api.py
