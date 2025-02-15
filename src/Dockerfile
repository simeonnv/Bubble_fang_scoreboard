FROM rust as build

# create a new empty shell project
RUN USER=root cargo new --bin scoreboard_backend
WORKDIR /scoreboard_backend

# copy over your manifests
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/scoreboard_backend*
RUN cargo build --release

# our final base
FROM rust

# copy the build artifact from the build stage
COPY --from=build /scoreboard_backend/target/release/scoreboard_backend .

# set the startup command to run your binary
CMD ["./scoreboard_backend"]