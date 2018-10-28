# Learning Rust

Following the [2018 Book](https://doc.rust-lang.org/book/2018-edition/index.html)

# Helpful Commands

If you have docker installed you can add projects root folder to your path, allowing you to type `ru <commands that use rust things>`.

```
ru cargo new guessing_game --vcs none
```
Create a new cargo project without git/vcs
```
ru cargo check
ru cargo build
ru cargo build --release
```
Various builds and releases. Each project will have its own cache for the build/libraries.
