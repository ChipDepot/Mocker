# Load rust image 
FROM rust:latest as builder

# Create new project
RUN USER=root cargo new --bin mocker
WORKDIR /mocker


# Copy the files
COPY ./mocker/Cargo.toml ./Cargo.toml
COPY ./mocker/src ./src

# Install cmake
RUN apt-get update
RUN apt-get install -y cmake
RUN cmake --version

# Build the app with the release flag
RUN cargo build --release

# Create a ligheter image using debian
FROM debian:buster-slim


# Copy the bin
COPY --from=builder /mocker/target/release/mocker /app

# Run mocker
CMD [ "/app/mocker" ]

