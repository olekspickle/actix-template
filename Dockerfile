# Run detached and remove container when it stopped
# Warning: Docker logs can mess up current terminal pane if not detached
#
# docker build -t actix-template:local .
# docker run -d -p 7777:7777 --rm --name actix-template --hostname actix-template

############################CACHE##############################################

FROM docker.io/rust:slim AS builder

# it is common to name cached image `build` but this messes up
# some frameworks fileserver which is configured in compile-time,
# so the build image and resulting image WORKDIR should match
WORKDIR /app

# copy the project
COPY . .

# 1. install stable Rust
# 2. run release build with cached rustup, cargo registry and target build artifacts
# 3. copy release binary with compressed debug symbols to the root
RUN --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/local/rustup \
    set -eux; \
    rustup install stable; \
    cargo build --release; \
    # in case you don't do that in cargo config, you can strip debug symbols here
    objcopy --compress-debug-sections target/release/actix-template ./actix-template

# ----------------------------------------------------------------------------------

FROM docker.io/debian:stable-slim

WORKDIR /app

# copy static and main server binary files
COPY --from=builder /app/static ./static
COPY --from=builder /app/actix-template ./actix-template
CMD ./actix-template
