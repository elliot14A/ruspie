# Build stage
FROM instrumentisto/rust:nightly-bullseye AS builder

WORKDIR /ruspie
COPY ./ /ruspie

RUN apt-get update \
    && apt-get install -y cmake \
    && cargo +nightly install --locked --git https://github.com/elliot14A/ruspie --bins ruspie

# Final stage
FROM debian:bullseye-slim

RUN apt-get update \
    && apt-get install -y libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/ruspie /usr/local/bin/ruspie

EXPOSE 8080

ENTRYPOINT ["ruspie"]
