# Rss2Email

[![Build & Tests](https://github.com/AntoniosBarotsis/Rss2Email/actions/workflows/ci.yml/badge.svg)](https://github.com/AntoniosBarotsis/Rss2Email/actions/workflows/ci.yml)
[![GitHub milestone](https://img.shields.io/github/milestones/progress/AntoniosBarotsis/rss2email/1?color=32ca55&label=Progress%20towards%20v1.0&labelColor=353d46)](https://github.com/users/AntoniosBarotsis/projects/2/views/1?query=is%3Aopen+sort%3Aupdated-desc)

A small program capable of aggregating content from multiple RSS/Atom feeds, and mail them to you in a practical summary email.  
Keep track of your favourite blogs that don't feature an update newsletter
or similar service.

* **Technology stack**: Rust, Docker, AWS Lambdas
* **Status**: Alpha

<p align="center">
  <img src="assets/res.jpg" alt="Example">
</p>

## Dependencies

You'll need [Rust](https://rust-lang.org/) installed to compile this software.

## Installation

Current mean of installation is compilation by source code.  
Clone this repository and run:

```bash
cargo build --release
```

Check out the [build from source](https://github.com/AntoniosBarotsis/Rss2Email/wiki/1.-Home#building-from-source) section from the wiki for more detailed informations.

## Configuration

**Rss2email** requires some environment variables in order to work, here listed.
These can be provided with any means, including an `.env` file

### `EMAIL_ADDRESS`

the mail address you will receive the feed content  

### `DAYS`

this value indicates up to how many days in the past we go to search for entries  

### `FEEDS`

a list of semicolon-separated feed URLS.  
_eg:_ `"https://blog.rust-lang.org/feed.xml;https://www.linux.org/articles/index.rss"`

### `EMAIL` (optional, defaults to `SENDGRID`)

Which provider 
Defaults to `SENDGRID`, can contain `EMAIL_COMMAND` as an alternative if you have `mail` or `sendmail` installed in your system  

### `API_KEY` (optional)

When using [SENDGRID](https://sendgrid.com/) you need one authentication key.

More details are available at wiki section [Running the code](https://github.com/AntoniosBarotsis/Rss2Email/wiki/3.-Running-the-Code)

## Usage

After setting up the configuration it will be possible to run the program

```bash
./target/release/Rss2email
```

Launching the program with cargo, with `dev` flags enabled, no email will be sent.  
Instead, the email content will be printed to `stdout`

```bash
cargo run
```

## Known Issues

### Slowness

Currently, the RSS feeds are downloaded in a sequential, blocking, non-concurrent manner as they are
only small text files and should thus not take too long (plus this is not supposed to run that often but
rather something like once a week or so).  

If you find yourself either using a *lot* of RSS feeds or really big ones somehow, do give me a heads up
by submitting an issue and I'll do what I can to make this faster. As of now, there is no reason to do that.
*That said, I might randomly decide to do this regardless*.

## Contributing

Thanks for considering contributing!

Read [this](./CONTRIBUTING.md).
