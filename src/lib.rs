#![allow(non_snake_case)]

use std::{error::Error, fs, env};

impl Config
{

//stdlib documentation for env::args() fn shows that the type of the iterator it returns is
//std::env::Args, and that type implements the Iterator trait and returns String values
//here, args can be any type that implements the Iterator trait and returns String items.
//alt: pub fn build(mut args: impl Iterator<Item = String>,) -> Result<Config, &'static str>

    pub fn build
    <Q> (mut args: Q) -> Result<Config,&'static str>
    where
    Q: Iterator<Item = String>
    {

        args.next();

        let query: String = match args.next()
        {
            Some(arg) => arg,
            None      => return Err("Didn't get no query string"),
        };

        let filePath: String = match args.next()
        {
            Some(arg) => arg,
            None      => return Err("Didn't get no file path") ,
        };

        let ignoreCase: bool = env::var("IGNORE_CASE").is_ok();
        eprintln!("{ignoreCase}");
        Ok(Config { query, filePath, ignoreCase })

    }

}

pub fn run
(config: Config) -> Result< (), Box<dyn Error> >
{
    let contents:String = fs::read_to_string(config.filePath)?;

    let results:Vec<&str> = if config.ignoreCase { searchCaseInsensitive(&config.query, &contents) }
    else { search(&config.query, &contents)}; //Ok, wow. But xuld be expected since if is an expression?!

    for line in results
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

/* finito

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
        let ignoreCase: bool = env::var("IGNORE_CASE").is_ok();
        eprintln!("{ignoreCase}");
        Ok(Config { query, filePath, ignoreCase })
    }
}

finito*/

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

pub struct Config
{ pub query: String, pub filePath: String, pub ignoreCase: bool,}
