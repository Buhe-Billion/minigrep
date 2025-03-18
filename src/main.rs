#![allow(non_snake_case)]

use std::{env, process};
use minigrep::Config;

fn main
()
{
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

//    let (query,filePath): (&str, &str) = parseConfig(&args);

    let config: Config = Config::build(&args)
    .unwrap_or_else
    (
        |err|
        {
            println!("Problem parsing arguments: {err}");
            process::exit(1);
        }
    );

    println!("Search for {0}",config.query);
    println!("In file {0}",config.filePath); //The 0 here is an index!!??!! wow!

/*
// fs::read_to_string opens a file and returns std::io::Result<String>
    let contents:String  = fs::read_to_string(filePath)
    .expect("Should have been able to read the file");
*/

    if let Err(e) = minigrep::run(config)
    {
        println!("App error: {e}");
        process::exit(1);
    }
}

/*
fn parseConfig
//&[String] reps a Vec<String> element borrow?!?! and maybe other collections with elements of String?
(args: &[String]) -> (&str, &str)
//(args: &str) was my initial set up, but we can't index into str with an int, UTF-8!
{
    let query:&str = &args[1];
    let filePath:&str = &args[2];

    (query,filePath)
}
*/
