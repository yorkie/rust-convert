
#![crate_id = "convert#0.0.1"]
#![crate_type = "lib"]

extern crate collections;
use collections::string::String;
use std::str;
use std::int;

#[inline]
pub fn string_to_str<'a>(raw: &'a String) -> Option<&'a str> {
  str::from_utf8((&raw).as_bytes())
}

#[inline]
pub fn str_to_int(raw: &str, radix: uint) -> Option<int> {
  int::parse_bytes(raw.as_bytes(), radix)
}

#[test]
fn test_string_to_str() {
  let s1 = String::from_str("foobar");
  let s2 = string_to_str(&s1);
  assert_eq!(s2, Some("foobar"));
}

#[test]
fn test_str_to_int() {
  assert_eq!(str_to_int("123", 10), Some(123));
}