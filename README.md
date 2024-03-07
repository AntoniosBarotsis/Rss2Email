# Rss2Email

[![Build & Tests](https://github.com/AntoniosBarotsis/Rss2Email/actions/workflows/ci.yml/badge.svg)](https://github.com/AntoniosBarotsis/Rss2Email/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/rss2email)](https://crates.io/crates/rss2email)
[![docs.rs](https://img.shields.io/docsrs/rss2email)](https://docs.rs/rss2email/)
![Minimum Supported Rust Version](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AntoniosBarotsis/87883f70db3cf998342786f65fe1b9df/raw/rss2email_msrv.json)
[![dependency status](https://deps.rs/repo/github/AntoniosBarotsis/Rss2Email/status.svg?path=.)](https://deps.rs/repo/github/AntoniosBarotsis/Rss2Email?path=.)
[![Docker Images](https://img.shields.io/badge/Docker-Images-0092e6?logo=docker)](https://hub.docker.com/search?q=antoniosbarotsis%2Frss2email)
[![Actively Maintained](https://img.shields.io/badge/Maintenance%20Level-Actively%20Maintained-green.svg)](https://gist.github.com/cheerfulstoic/d107229326a01ff0f333a1d3476e068d)
<!-- [![GitHub milestone](https://img.shields.io/github/milestones/progress/AntoniosBarotsis/rss2email/1?color=32ca55&label=Progress%20towards%20v1.0&labelColor=353d46)](https://github.com/users/AntoniosBarotsis/projects/2/views/1?query=is%3Aopen+sort%3Aupdated-desc&filterQuery=milestone%3A%22v1.0%22) -->

A small program capable of aggregating content from multiple RSS/Atom feeds and mailing them to you
in a practical summary email. Keep track of your favorite blogs that don't feature an update
newsletter or similar service.

<p align="center">
  <img src="assets/res.jpg" alt="Example">
</p>

## Dependencies

You'll need [Rust](https://rust-lang.org/) or [Docker](https://www.docker.com/) installed to
compile this software.

## Installation

Each release automatically publishes Docker images for
[x86](https://hub.docker.com/repository/docker/antoniosbarotsis/rss2email-x86) and
[arm](https://hub.docker.com/repository/docker/antoniosbarotsis/rss2email-arm) on DockerHub.
Note that these can only run on AWS Lambda. If you want to run them elsewhere through Docker, read
[here](https://github.com/AntoniosBarotsis/Rss2Email/wiki/4.-More-on-Docker).

Alternatively, you can build this from source. Clone this repository and run:

```bash
cargo build --release
```

Check out the
[build from source](https://github.com/AntoniosBarotsis/Rss2Email/wiki/1.-Home#building-from-source)
section of the wiki for more information.

## Configuration

**Rss2email** requires some environment variables to work. These can be provided either in your
shell or as entries in a `.env` file.

- `EMAIL_ADDRESS`: the mail address you will use to send the emails
- `RECIPIENT_ADDRESSES`: comma delimited list of recipient email addresses
- `SUBJECT`: the email subject (could be `rss2email`). Including `$POST_COUNT` in your subject will
  have it replaced with the number of posts fetched. For example `SUBJECT=rss2email - $POST_COUNT new posts`
- `DAYS`: this value indicates up to how many days in the past we go to search for entries  
- `FEEDS`: a list of semicolon-separated feed URLs.  
  _eg:_ `"https://blog.rust-lang.org/feed.xml;https://www.linux.org/articles/index.rss"`
- `EMAIL` (optional, defaults to `SendGrid`):  Which provider to use to send the email.  
  For the supported providers, you can check the 
  [docs](https://docs.rs/rss2email/latest/rss2email_lib/email/email_provider/enum.EmailProviders.html).
- `API_KEY` (optional): Your email provider's authentication key.
- `SKIP_IF_NO_NEW_POSTS` (optional): Whether an email should be sent if the number of posts fetched is 0.

More details are available in the 
[Running the code](https://github.com/AntoniosBarotsis/Rss2Email/wiki/3.-Running-the-Code) wiki 
section.

## Usage

Running the code in debug mode won't send any emails and will instead output the generated HTML in
the console.

```bash
cargo run
```

It is recommended to try this out first and make sure that all your feeds and config variables are
correctly set up.

Running the project in release mode will send the emails

```bash
./target/release/Rss2email
# or
cargo run --release
```

<!-- ## Known Issues -->

## Contributing

Thanks for considering contributing!

Read [this](./CONTRIBUTING.md).

## Thanks to all Contributors!

<a href="https://github.com/AntoniosBarotsis/Rss2Email/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=AntoniosBarotsis/Rss2Email" />
</a>
