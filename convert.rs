
#![crate_id = "convert#0.0.1"]
#![crate_type = "lib"]

extern crate collections;
use collections::string::String;
use std::str;

#[inline]
pub fn string_to_str<'a>(raw: &'a String) -> Option<&'a str> {
  str::from_utf8((&raw).as_bytes())
}

#[test]
fn test() {
  let s1 = String::from_str("foobar");
  let s2 = string_to_str(&s1).unwrap();
  assert_eq!(s2, "foobar");
}
