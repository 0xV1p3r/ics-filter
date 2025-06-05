FROM rust:alpine AS builder

WORKDIR /app

#Copy the application into the container.
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN ["apk", "add", "musl-dev", "openssl", "openssl-dev", "openssl-libs-static", "--no-cache"]
RUN ["cargo", "build", "--release"]

FROM alpine:3.16

#Install Caddy
RUN ["apk", "update"]
RUN ["apk", "add", "caddy", "--no-cache"]

#Setup cron
RUN ["apk", "add", "busybox-initscripts", "openrc", "--no-cache"]
RUN ["sh", "-c", "crontab -l | { cat; echo \"30 * * * * cd /app && ./ics-filter\"; } | crontab -"]

WORKDIR /app
RUN ["mkdir", "calendar_serving"]
COPY --from=builder /app/target/release/ics-filter .
RUN ["chmod", "a+x", "./ics-filter"]

CMD ["sh", "-c", "crond -f & caddy file-server --listen :80 --root ./calendar_serving"]
