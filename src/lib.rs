#![allow(non_snake_case)]

use std::{error::Error, fs};

pub fn run
(config: Config) -> Result< (), Box<dyn Error> >
{
    let contents:String = fs::read_to_string(config.filePath)?;

    for line in search(&config.query, &contents)
    { println!("{line}"); }
    Ok(())
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn caseSensitive
    ()
    {
        let query: &'static str = "duct";
        let contents: &'static str =
        "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn caseInsensitive
    ()
    {
        let query: &'static str = "rUsT";
        let contents: &'static str =
        "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!
        (
            vec!["Rust:","Trust me."],
            searchCaseInsensitive(query, contents)
        );
    }
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

pub fn search
<'a> (query: &str, contents:&'a str) -> Vec<&'a str>
{
    let mut results: Vec<&'a str> = Vec::new();

    for line in contents.lines()
    {
        if line.contains(query)
        { results.push(line); }
    }

    results
}

pub fn searchCaseInsensitive
<'a> (query:&str, contents:&'a str,) -> Vec<&'a str>
{
    let query: String = query.to_lowercase();
    let mut results: Vec<&'a str> = Vec::new();

    for line in contents.lines()
    {
//ampersand to query coz contains() is defined as taking a string slice
        if line.to_lowercase().contains(&query)
        { results.push(line); }
    }

    results
    //fn is not water proof according to Unicode standards!
}
