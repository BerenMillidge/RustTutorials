// our own implementation ofthe command line tool grep!!! that is really cool
// apparently rust shuold be really good for these sorts of things... so that is nice!
// so first thing first is to be able to accept command line arguments get that working

use std::env;
use std::fs;
use std::io::prelude::*;
use std::process;
use std::error::Error;

struct Config {
	query: String,
	filename: String,
}
// create parse config functino on the config struct/object
impl Config {
    fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    	args.next();
    	// skip the first argument

    	let query = match args.next() {
    		Some(arg) => arg,
    		None => return Err("DIdn't get a query string"),
    	};
    	let filename = match args.next() {
    		Some(arg) => arg,
    		None => return Err("DIdn't get a file name"),
    	};

    	let case_sensitive = env::var("Case insensitive").is_err();

    	Ok(Config {query, filename, case_sensitive})

    	// uses the new iterator to get the config which makes sense and is awesome... but who knwos really!
      
	}
}
fn parse_config(args: &[String]) ->Config {
	let query = args[1].clone();
	let filename = args[2].clone();
	return Config {query, filename};
}

// now extract all the other logic into a runfunction

fn run(config:Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(config.filename)?;
	println!("{}", contents);

	for line in search(&config.query, &contents) {
		println!("{}", line);
	}
	Ok(())
}

// implement base search function
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	// so very simple serch, just iterate through the liens
	// check whether the line is the query string
	// add it i so don't do anything else else
	let mut results = Vec::new();

	for line in contents.lines() {
		if line.contains(query) {
			results.push(line);

		}
	}
	return results;
}

// can also rewrite this with iterators in a more clear - i.e. fucntional way
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	contents.lines()
		.filter(|line| line.contains(query))
		.collect()
}
// that is reall quite simple once you know how to read the iterators!
// iterators are a zero cost abstraction. They mostly get compiled away, so don't worry about using that
// and to be honest, overoptimisation is the root of a lot of evil anyway, so it doesn't really matter!?


fn main() {
	let args: Vec<String> = env::args().collect();
	println!("{:?}", args);

	//let query = &args[1];
	//let filename = &args[2]; // you don't want to take ownership of the arg sfunction here else future accesses won't work!
	
	let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

	println!("Searching for {}", config.query);
	println!("In file {}", config.filename);

	// use if let for error handling
	if let Err(e) = run(config) {
		println!("Application Error: {}", e);
		process::exit(1);
		// can also print to standard error instead using eprintln macro
		eprintln!("Problem parsing arguments: {}", e);
	}

	
}

// now implement TDD forthe rest of the searching logic!
// assert failing test
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}