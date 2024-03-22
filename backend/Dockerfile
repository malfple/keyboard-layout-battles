FROM rust:latest as build

# Create a new empty shell project
RUN USER=root cargo new --bin klb-backend
WORKDIR /klb-backend

# Copy manifest
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy source
COPY ./src ./src

# build
RUN rm ./target/release/deps/klb_backend*
RUN cargo build --release

# final base image
FROM rust:slim

# copy artifact from build stage and some other files
COPY --from=build ./klb-backend/target/release/klb-backend .

# setup settings + other files
COPY ./settings/SettingsDocker.toml ./settings/Settings.toml
COPY ./wordlist.json ./wordlist.json

# run the binary
CMD ["./klb-backend"]