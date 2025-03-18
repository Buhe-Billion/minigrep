#![allow(warnings)]

use std::{env, fs};

fn main
()
{
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let (query,filePath): (&str, &str) = parseConfig(&args);

    println!("Search for {query}");
    println!("In file {filePath}");

    let contents:String  = fs::read_to_string(filePath)           // fs::read_to_string opens a file and returns std::io::Result<String>
    .expect("Should have been able to read the file");

    println!("With text: \n {contents}");
}

fn parseConfig
//&[String] reps &Vec<String>?!?! and maybe other collections with elements of String?
(args: &[String]) -> (&str, &str)
//(args: &str) was my initial set up, but we can't index into str with an int, UTF-8! 
{
    let query:&str = &args[1];
    let filePath:&str = &args[2];

    (query,filePath)
}
