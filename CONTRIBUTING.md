# Contributing

## Issues

### My feed isn't working, what's wrong?

Turns out not all websites abide the [official RSS specification](https://www.rssboard.org/rss-specification).
An example of one is none other than [the offician Rust blog](https://blog.rust-lang.org/feed.xml)!

I decided to try and support different formats as they seem to be adequately consistent. Of course, I cannot
know every single different format that websites might decide to use beforehand so this is where you come in!

If you get an error that resembles something along the lines of

```log
[WARN] Error in (your feed's link) ["custom: missing field `...`"]
```

Then there's a good chance that you found a feed type I do not support yet, make sure to open an issue
and include the link to your RSS feed!

### The Dates are Wrong!

I have made some strong assumptions (that seem to be correct in the feeds I tried) about the dates so 
Getting that wrong is almost expected. Submit a bug report!

More specifically, dates in feeds that use the actual RSS specification seem to be formatted according to
the [RFC 2822](https://www.rfc-editor.org/rfc/rfc2822#section-3.3) specification while the other feed format
I came across (same one used in the Rust Blog) seems to be formatted in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
according to [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339#section-5).

### Something Else is Wrong!

I'm very new to rust (as in been doing this for 2 weeks now) and that plus the fact that I am overengineering
some aspects of the project purely for exploring the language means that there will very likely be a lot of 
random bugs baked in the project.

I would really like to fix any potential issues that my code might have so make sure to submit bug reports
preferably with reproducable examples!

## Contributing to the Codebase

Make sure to detail your desired changes in either an issue or a discussion first, I will try to reply to them
and then you can go ahead and start working on your fork.

The way the whole different feed format is a bit weird so let me explain how that works.

I currently have 2 supported formats which correspond to the [RSS](./src/xml/rss.rs) and 
[Feed](./src/xml/feed.rs) structs in the code respectively.

Both follow a similar structure; They have a struct that represents the root of the document and another
struct that represents the posts. These need to be converted to the internal representation of a feed
which I named [blog](./src/blog.rs). This is accomplished through some traits I define 
[here](./src/xml/traits.rs). All 3 of these traits need to be implemented for any new formats, you can check
both `rss` and `feed` for implementation examples.

That is done so using a new format is then extremely simple. The attempts at parsing the XML into one of those
formats is currently done [here](./src/xml/mod.rs). All you need to do is append your new format to the vector

```rs
let possible_roots = vec![
  from_str::<Rss>(&xml).into_blog(),
  from_str::<Feed>(&xml).into_blog(),
];
```

This is probably far from a good solution but as I said earlier, I wanted to experiment with the language
and it does work so...

As a last note, if you are planning on contributing a new feed format, please consider leaving a comment similar to those
at the start of both [`feed.rs`](./src/xml/feed.rs) and [`rss.rs`](./src/xml/rss.rs) for better maintainability.
