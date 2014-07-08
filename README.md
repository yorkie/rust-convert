
rust-convert
================
convienient api for type converting for Rust

### Installation

by `Cargo.toml`:
```toml
[dependencies.imap]
git = "https://github.com/yorkie/rust-convert"
```

### Usage
```rs
extern crate convert;
extern crate collection;
use collection::String;

let raw: String = String::from_str("foobar");
let str: &str = convert::str_to_string(&raw).unwrap();
```

### License

MIT
