#![allow(non_snake_case)]

use std::{error::Error, fs};

pub fn run
(config: Config) -> Result< (), Box<dyn Error> >
{
    let contents:String = fs::read_to_string(config.filePath)?;

    println!("With text: \n {contents}");

    Ok(())
}

impl Config
{
    pub fn build
    //We can return a Config instance or a string literal
    (args: &[String]) -> Result<Config, &'static str>
    {
        if args.len() < 3
        { return Err("not enough arguments"); }

        let query: String = args[1].clone();
        let filePath: String = args[2].clone();

        Ok(Config { query, filePath })
    }
}

pub struct Config
{ pub query: String, pub filePath: String, }
