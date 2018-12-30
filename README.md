# Learning Rust

Following the [Book](https://doc.rust-lang.org/book/index.html)

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

## How do I exit the container?
Because the containers run in interactive mode, you'll have to do Control-P Control-Q to detach
