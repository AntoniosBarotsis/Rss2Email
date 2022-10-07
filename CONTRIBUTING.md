# Contributing

## Running the Code

I go into more detail [in the Wiki](https://github.com/AntoniosBarotsis/Rss2Email/wiki/3.-Running-the-Code) but basically, you can just

- Create a `feeds.txt` file with some links (see [here](https://github.com/AntoniosBarotsis/Rss2Email/wiki/2.-Setting-up-the-emails) for example)
- `cargo run`

You should see HTML dumped in your console.

## Issues

### My feed isn't working, what's wrong?

I based my implementation around the [official RSS specification](https://www.rssboard.org/rss-specification) and the 
[Atom Syndication Format](https://www.rfc-editor.org/rfc/rfc4287), if you find a particular feed that seems to be causing issues
there might be an inconsistency on their part (or in my implementation) or it might just be that some of the optional attributes
that I consider necessary (a link or a date for example) are not present. 

In any case, open an issue and I'll get back to you :)

### Something Else is Wrong!

I'm very new to rust (as in been doing this for 2 weeks now) and that plus the fact that I am overengineering
some aspects of the project purely for exploring the language means that there will very likely be a lot of 
random bugs baked in the project. That said, I have been running the code since day 0 and if I do
find any bugs, you will be able to see them 
[here](https://github.com/AntoniosBarotsis/Rss2Email/issues?q=is%3Aopen+label%3Abug+sort%3Aupdated-desc).

I would really like to fix any potential issues that my code might have so make sure to submit bug reports
preferably with reproducable examples!

## Contributing to the Codebase

Make sure to detail your desired changes in either an issue or a discussion first, I will try to reply to them
and then you can go ahead and start working on your fork.

The way the whole different feed format is a bit weird so let me explain how that works.

I currently have 2 supported formats which correspond to the [RSS](./src/xml/rss.rs) and 
[Atom](./src/xml/atom.rs) structs in the code respectively.

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
