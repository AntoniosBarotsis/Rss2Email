# Contributing

## Running the Code

I go into more detail 
[in the Wiki](https://github.com/AntoniosBarotsis/Rss2Email/wiki/3.-Running-the-Code) but basically,
you can just

- Create a `.env` file with your configs (see 
  [here](https://github.com/AntoniosBarotsis/Rss2Email/wiki/3.-Running-the-Code#creating-the-env-file))
- Create a `feeds.txt` file with some links (see 
  [here](https://github.com/AntoniosBarotsis/Rss2Email/wiki/3.-Running-the-Code#adding-rss-feeds))
- `cargo run`

You should see HTML dumped in your console.

## Potentially Interesting Parts of the Code

- Email stuff happens in the [`email`](./src/email/) folder. For an example implementation,
  you can check [`sendgrid`](./src/email/sendgrid.rs). The implementations are aggregated
  [here](./src/email/email_provider.rs)
- Feed parsing happens in the [`xml`](./src/xml/) folder and specifically in the
  [`mod.rs`](./src/xml/mod.rs) file.
- [`blog.rs`](./src/blog.rs) contains the internal representations of the web feeds.
- [`lib.rs`](./src/lib.rs) contains most of the "business logic" of the project which is called from
  [`main.rs`](./src/main.rs) that in turn contains some boiler-plate needed to spin the code up
  both locally and on AWS Lamda.

## Issues

### My feed isn't working, what's wrong?

I based my implementation around the
[official RSS specification](https://www.rssboard.org/rss-specification) and the
[Atom Syndication Format](https://www.rfc-editor.org/rfc/rfc4287), if you find a particular feed
that seems to be causing issues there might be an inconsistency on their part (or in my 
implementation) or it might just be that some of the optional attributes that I consider necessary
(a link or a date for example) are not present. 

In any case, open an issue and I'll get back to you :)

### Something Else is Wrong!

I'm very new to rust (as in been doing this for 2 weeks now) and that plus the fact that I am
overengineering
some aspects of the project purely for exploring the language means that there will very likely be a
lot of random bugs baked in the project. That said, I have been running the code since day 0 and if
I do find any bugs, you will be able to see them 
[here](https://github.com/AntoniosBarotsis/Rss2Email/issues?q=is%3Aopen+label%3Abug+sort%3Aupdated-desc).

I would really like to fix any potential issues that my code might have so make sure to submit bug
reports preferably with reproducible examples!

## Contributing to the Codebase

Make sure to detail your desired changes in either an issue or a discussion first, I will try to
reply to them, and then you can go ahead and start working on your fork.

The way the whole different feed format is a bit weird so let me explain how that works.

I wrote a pretty lengthy blog post on the project
[here](https://antoniosbarotsis.github.io/posts/rss2email/) where I explain most things in depth
so feel free to use that as a sort of external documentation if you have questions about some
part of the project. If you don't find what you're looking for then make sure to open a
[discussion](https://github.com/AntoniosBarotsis/Rss2Email/discussions/new) and I'll do my best
to explain it to you.
