# Larger Debian-based Rust prebuilder image
FROM rust as builder
WORKDIR /app
COPY Cargo* /app/
COPY src /app/src/
RUN cargo install --path .

# :latest as of Dec 2023
FROM busybox:1.35.0-glibc
RUN mkdir -p /srv/arthurk.com/
COPY --from=builder /usr/local/cargo/bin/webserver /srv/webserver
ADD *.html *.css *.jpg /srv/arthurk.com/
CMD ["./webserver"]
