# AOC 2024 in Rust

## Helpers

Running the program expects the session cookie to be set in the environment variable `AOC_SESSION` so we can fetch your input data. \
In order to do that, you can run the following command in your terminal:
 ```bash
export AOC_SESSION=you-session
```

Starting the program is done by running the following command:

```bash
cargo run -- 1 2 --sample
```

 - first arguments is the day
 - second argument is the puzzle
 - `--sample` is the flag to use the sample input, if omitted the real input file is used

You can start a watcher for development by running the following command:

```bash
cargo watch -x check -x "run -- 1 2 --sample"
```
