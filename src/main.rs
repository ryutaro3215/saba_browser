#![no_std]
#![no_main]

extern crate alloc;

use noli::prelude::*;
use net_wasabi::http::HttpClient;
use crate::alloc::string::ToString;

fn main() -> u64 {
    let client = HttpClient::new();
    match client.get("host.test".to_string(), 8000, "/test.html".to_string()) {
        Ok(res) => {
            print!("response:\n{:#?}", res);
        }
        Err(e) => {
            print!("error:\n{:#?}", e);
        }
    }
    0
}


entry_point!(main);
