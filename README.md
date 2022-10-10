# Rss2Email

[![Build & Tests](https://github.com/AntoniosBarotsis/Rss2Email/actions/workflows/ci.yml/badge.svg)](https://github.com/AntoniosBarotsis/Rss2Email/actions/workflows/ci.yml)
![GitHub milestone](https://img.shields.io/github/milestones/progress/AntoniosBarotsis/rss2email/1?color=32ca55&label=Progress%20towards%20v1.0&labelColor=353d46)

This project collects blog posts made over the last `n` days from RSS feeds and emails them to you!

<p align="center">
  <img src="assets/res.jpg" alt="Example">
</p>

## Why?

I have a few blogs in mind that I do find interesting but they do not provide a newsletter.

There are some RSS readers but I am too lazy to download and use other software exclusively for this,
I would much rather see a summary of these posts in my mailbox.

## Getting Started

This section was getting rather lengthy so I moved it to [the Wiki](https://github.com/AntoniosBarotsis/Rss2Email/wiki/1.-Home)!

### ⚠ Docker ⚠

Please read [this](https://github.com/AntoniosBarotsis/Rss2Email/wiki/1.-Home#-important-) if you are planning on building the docker image.

## Why is this so Slow?

Currently, the RSS feeds are downloaded in a sequential, blocking, non-concurrent manner as they are 
only small text files and should thus not take too long (plus this is not supposed to run that often but
rather something like once a week or so). 

If you find yourself either using a *lot* of RSS feeds or really big ones somehow, do give me a heads up
by submitting an issue and I'll do what I can to make this faster. As of now, there is no reason to do that.
*That said, I might randomly decide to do this regardless*.

## Contributing

Thanks for considering contributing!

Read [this](./CONTRIBUTING.md).
