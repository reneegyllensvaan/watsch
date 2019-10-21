# Watsch - A little file watcher CLI
This is a little CLI i wrote to run a command any time one of one or more files is saved. It has no command line options.

## Usage
```rust
watsch FILE1 [FILE2, ...] -- COMMAND [ARGUMENTS]
```

## Example
Watch `main.rs` and `my_module` (a directory) for changes. When a change is
registered, run `sh -c 'clear && cargo run'`, i.e. clear the screen and run
the application.
```rust
watsch main.rs -- sh -c 'clear && cargo run'
```

## Setup
```
cargo install --git https://github.com/reneegyllensvaan/watsch.git
```
