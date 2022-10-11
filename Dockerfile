ARG  base_image=rust
FROM ${base_image}:1.64-alpine as builder
# rust or arm64v8/rust

# Read https://github.com/AntoniosBarotsis/Rss2Email/wiki/1.-Home#deploying
#
# TLDR; run docker build with `--build-arg compile_flag="--features aws-lambda"`
# if you want to build for Lambda
ARG compile_flag=""

# RUN apk add --no-cache musl-dev
WORKDIR /opt
RUN cargo new --bin rss2email
WORKDIR /opt/rss2email
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
ADD ./benches ./benches
RUN cargo build --release $compile_flag

RUN rm ./src/*.rs
RUN rm ./target/release/deps/rss2email*

ADD ./src ./src
RUN cargo build --release $compile_flag

FROM scratch
WORKDIR /opt/rss2email
COPY --from=builder /opt/rss2email/target/release/rss2email .
COPY *.env ./.env
COPY *feeds.txt ./feeds.txt
CMD ["./rss2email"]
