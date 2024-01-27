

use std::error::Error;
use std::fs;
use std::env;


pub struct Config {
   pub query: String,
     pub filename: String,
}

 impl Config {
   pub  fn new (args : &[String]) -> Result< Config, &str> {
        if args.len() < 3{
        return Err("too many arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config{query, filename})
    }
}
pub fn run (config:Config)-> Result<(), Box <dyn Error>> {
    let contents  = fs::read_to_string(config.filename)?;

    for line in contents.lines() {
        println!("
        {}", line);
        if line.contains("frog"){
            print!("Match found {}", line)
        }
    }

    // println!("The contents is \n{}", contents);
    Ok(())
}
fn parse_config(args: &[String]) -> Config{
    let query = args[1].clone();
    let filename = args[2].clone();
    Config{query, filename}
}
pub fn search_case_sensitive<'a>(query: &str, contents: &'a  str) -> Vec<&'a str>{

    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(query) {
results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests{
    use super::*;

    // Add test attribute
    #[test]
    fn one_result(){
        let query = "safe";
        let contents = "\n
        Rust:\nsafe, fast, productive.\n
        Pick three";
        println!("{:?}aaaaaa",search_case_sensitive(query, contents));
        assert_eq!(vec!["safe, fast, productive."],search_case_sensitive           (query, contents))
    }
     
}