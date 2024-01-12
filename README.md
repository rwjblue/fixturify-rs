# fixturify-rs

Attempt at a Rust port of [fixturify](https://github.com/joliss/node-fixturify) NPM package. The high level goal is to provide a way to create a directory structure in a test without having to create each individual file and directory imperatively.

The most common use will be to leverage the `fixture!` macro like:

```rust
let macro_fixture = fixture! {
  "fileHere.txt" => "other contents",
  "otherFile" => "lol yesss",
  "subdirThere" => {
    "subdirFile" => "subdir contents",
  },
};
```
