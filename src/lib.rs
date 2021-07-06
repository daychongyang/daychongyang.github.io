use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
      return Err("not enough arguments.");
    }

    let query = args[1].clone();
    let filename = args[2].clone();

    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    Ok(Config {
      query,
      filename,
      case_sensitive,
    })
  }
}

pub fn search_case_sensitive<'a>(
  query: &str,
  contents: &'a str,
) -> Vec<&'a str> {
  let mut result = Vec::new();
  for line in contents.lines() {
    if line.contains(query) {
      result.push(line);
    }
  }

  result
}

pub fn search_case_insensitive<'a>(
  query: &str,
  contents: &'a str,
) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut result = Vec::new();
  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      result.push(line);
    }
  }

  result
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  let result = if config.case_sensitive {
    search_case_sensitive(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };

  for line in result {
    println!("{}", line);
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
    ";

    assert_eq!(
      vec!["safe, fast, productive."],
      search_case_sensitive(query, contents)
    )
  }

  #[test]
  fn case_insensitive() {
    let query = "rUst";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.
    ";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)
    )
  }
}
