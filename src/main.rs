#![allow(warnings)]

use std::env;

fn main
()
{
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let query: &str = &args[1];
    let filePath = &args[2];

    println!("Search for {query}");
    println!("In file {filePath}");
}
