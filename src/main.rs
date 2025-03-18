#![allow(warnings)]

use std::{env, fs};

fn main
()
{
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

//    let (query,filePath): (&str, &str) = parseConfig(&args);

    let config: Config = Config::new(&args);

    println!("Search for {0}",config.query);
    println!("In file {0}",config.filePath); //The 0 here is an index!!??!! wow!

/*
// fs::read_to_string opens a file and returns std::io::Result<String>
    let contents:String  = fs::read_to_string(filePath)
    .expect("Should have been able to read the file");
*/

    let contents:String = fs::read_to_string(config.filePath)
    .expect("Shoulda been able to read the file");

    println!("With text: \n {contents}");
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

impl Config
{
    fn new
    (args: &[String]) -> Config
    {
        let query: String = args[1].clone();
        let filePath: String = args[2].clone();

        Config {query, filePath }
    }
}

struct Config
{ query: String, filePath: String, }
