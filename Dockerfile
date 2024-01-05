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

# Build the app with the release flag
RUN cargo build --release

# Create a lighter image using debian
FROM ubuntu:latest

RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y openssl
RUN apt-get install -y build-essential

# Copy the bin
COPY --from=builder /mocker/target/release/mocker mocker

# Run mocker
ENTRYPOINT [ "./mocker" ]

