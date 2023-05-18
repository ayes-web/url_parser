<h1 align="center">URL Arguments parser</h1>

## Rust macro that parses url arguments into a struct for easy usage

# Install
```
cargo add --git https://github.com/ayes-web/url_parser
```

# Usage
## You can add multiple URLs, it will add flags from all of them

```rust
#[macro_use]
extern crate url_parser;

#[parse_url("https://nyaa.si/?f=0&c=1_0&q=q&u", "https://nyaa.si/?s=comments", "https://nyaa.si/?p=100", "https://nyaa.si/?page=rss")]
struct ParsedURL {}
```

↓↓↓↓↓↓↓↓↓↓

```rust
struct ParsedURL {
    pub f: String,
    pub page: String,
    pub u: Option<String>,
    pub q: String,
    pub s: String,
    pub p: String,
    pub c: String,
}
```
