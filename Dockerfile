# Use the official Rust image as a build stage
FROM rust as builder

# Create a new empty shell project
RUN USER=root cargo new --bin the-bubblehouse
WORKDIR /the-bubblehouse

# Copy our manifest and install dependencies
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code and build the application
COPY ./src ./src
RUN rm ./target/release/deps/the_bubblehouse*
RUN cargo build --release

# Use the official Rust image for the runtime
FROM rust

# Copy the build artifact from the build stage
COPY --from=builder /the-bubblehouse/target/release/the-bubblehouse /usr/local/bin/the-bubblehouse

# Expose the application port
EXPOSE 8080

# Run the binary
CMD ["the-bubblehouse"]

