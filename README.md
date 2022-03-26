# Books

Providing an input list of ISBNs fetches from Google Books the corresponding volumes and save them in a JSON simplified library file stored locally. It avoid to fetch the volumes multiple times if already stored.

Usage:

```bash
> books -h
```

## Compiling

Launch `cargo build --release` and move the bin around.

## Configuration

It automatically creates a config file under `~/.books/config.json`. You can create your own copy starting from `config.dist.json`. In this file you can set Google API key and library location.

```json
{
    "base_url":  "https://www.googleapis.com/books/v1/",
    "output" : "library.json",
    "api_key": "xxxxx"
}
```

Passing some ENV variables overwrites some settings:

- `BOOK_OUTPUT` env overwrites the library location.
- `BOOK_CONFIG` overwrite default configuration path.

## Input

The list of input ISBN can be passed either through a filename `-i filename` or as a list of ISBN or by stdin pipe. The filename format split by newline and space. ISBN should be numeric, but dashes are accepted.
