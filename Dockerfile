FROM rust:1.64-alpine as builder

# Read https://github.com/AntoniosBarotsis/Rss2Email/wiki#deploying
#
# TLDR; run docker build with `--build-arg compile_flag="--features aws-lambda"`
# if you want to build for Lambda
ARG compile_flag=""

RUN apk add --no-cache musl-dev
WORKDIR /opt
RUN cargo new --bin rss2email
WORKDIR /opt/rss2email
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release $compile_flag

RUN rm ./src/*.rs
RUN rm ./target/release/deps/rss2email*

ADD ./src ./src
RUN cargo build --release $compile_flag

FROM scratch
WORKDIR /opt/rss2email
COPY --from=builder /opt/rss2email/target/release/rss2email .
# Place configuring files in a dedicated folder
COPY ./.env ./config/.env
COPY ./feeds.txt ./config/feeds.txt

# Start the container from this dedicated folder
WORKDIR /opt/rss2email/config
CMD ["/opt/rss2email/rss2email"]
