FROM rust:latest

#Copy the application into the container.
COPY ./src /app/src
COPY ./Cargo.toml /app/Cargo.toml
COPY ./Cargo.lock /app/Cargo.lock

WORKDIR /app

RUN ["cargo", "build", "--release"]

RUN ["apt-get", "-y", "update"]
RUN ["apt-get", "install", "-y", "cron"]

# Add cron job
RUN ["touch", "/app/crontab.log"]
RUN crontab -l | { cat; echo "30 * * * * cd /app && /app/target/release/ics-filter >> /app/crontab.log 2>&1"; } | crontab -

# Run the application.
CMD cron -f 
