LABEL org.opencontainers.image.source https://github.com/excalq/arthurk.com

# Larger Debian-based Rust prebuilder image
FROM rust as builder
WORKDIR /app
COPY Cargo* /app/
COPY src /app/src/
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
COPY --from=builder /usr/local/cargo/bin/webserver /srv/webserver
ADD *.html *.css *.jpg /srv/
USER 1000
HEALTHCHECK --interval=30s --timeout=3s \
  CMD curl -f http://localhost/ || exit 1
WORKDIR /srv
CMD ["/srv/webserver"]
