FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y mailutils \
    && apt-get clean

COPY rss2email /usr/sbin
