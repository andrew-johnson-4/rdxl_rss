#![feature(proc_macro_hygiene)]
use rdxl::xhtml;
use rdxl_rss::*;

fn bs(s: String) -> String {
   s.split_whitespace().collect::<Vec<&str>>().join(" ")
}

#[test]
fn format1() {
   assert_eq!(
      bs(xhtml!(<!Rss/>)),
      r#"<?xml version="1.0" encoding="UTF-8" ?> <rss version="2.0"> </rss>"#
   );
}
