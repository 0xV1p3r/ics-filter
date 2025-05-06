FROM rust:latest AS builder

#Copy the application into the container.
COPY ./src /app/src
COPY ./Cargo.toml /app/Cargo.toml
COPY ./Cargo.lock /app/Cargo.lock
WORKDIR /app
RUN cargo install --path .

FROM debian:bullseye-slim

RUN apt-get -y update
RUN apt-get install -y python3 cron

WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/ics-filter /usr/local/bin/ics-filter

# Add cron job
RUN touch /app/crontab.log
RUN crontab -l | { cat; echo "30 * * * * cd /app && ics-filter config.toml >> /app/crontab.log 2>&1"; } | crontab -

# Run the application.
CMD cron -f & cd /app/ics_files && /usr/bin/python3 -m http.server 80 && echo 'Startup complete'
