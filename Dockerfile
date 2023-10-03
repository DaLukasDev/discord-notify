################
##### Builder
FROM rust:slim as builder

RUN apt-get update && apt-get install -y \
  musl-tools && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src

# Create blank project
RUN USER=root cargo new discord-notifier

# We want dependencies cached, so copy those first.
COPY Cargo.toml Cargo.lock /usr/src/discord-notifier/

# Set the working directory
WORKDIR /usr/src/discord-notifier

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-musl

# This is a dummy build to get the dependencies cached.
RUN cargo build --target x86_64-unknown-linux-musl --release

# Now copy in the rest of the sources
COPY src /usr/src/discord-notifier/src/

## Touch main.rs to prevent cached release build
RUN touch /usr/src/discord-notifier/src/main.rs

# This is the actual application build.
RUN cargo build --target x86_64-unknown-linux-musl --release

################
##### Runtime
FROM alpine:latest AS runtime 

# Copy application binary from builder image
COPY --from=builder /usr/src/discord-notifier/target/x86_64-unknown-linux-musl/release/discord-notifier /usr/local/bin

EXPOSE 3030

# Run the application
CMD ["/usr/local/bin/discord-notifier"]