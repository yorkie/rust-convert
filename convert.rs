
#![crate_id = "convert#0.0.1"]
#![crate_type = "lib"]

use std::str;

pub fn string_to_str(raw: String) -> &str {
  str::from_utf8(raw.as_bytes()).unwrap()
}
