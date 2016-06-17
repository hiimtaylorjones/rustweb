# What's Covered In This Tutorial

* Serialization in Rust
* Using Serialization for Diesel models
* Basic filtering and queries in Diesel

## ONE - Pulling from an API

### Why Are We Doing This?

Web-based API's are a huge part of modern software development, and chances are
that they here to stay. So, one of the first questions I asked myself when learning
Rust was: "How do I make an API work well with this language?"

While the amount of resources on the subject aren't as vast as something like
Rails, Laravel, or Node, what little I've seen from Rust's API functionality
is interesting.

The specific aspect of API's I want to talk about is reading from external API's.
Let's say that I have access to a blogging network's API. As a developer, I want
to pull post data from the source and log the blog posts into my own system for archival
purposes.

We're not going to care about security today, even though its important. I'm pulling
from a free and open-api at [http://jsonplaceholder.typicode.com/](http://jsonplaceholder.typicode.com/).

## Two - Serialization and Diesel

## Three - Basic Queries in Diesel
