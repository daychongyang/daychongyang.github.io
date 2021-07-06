use std::{env, process};

use daily::{run, Config};

fn main() {
  let args: Vec<String> = env::args().collect();

  let config = Config::new(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  println!("Search for {}", config.query);
  println!("In file {}", config.filename);

  if let Err(e) = run(config) {
    println!("Application error: {}", e);
    process::exit(1);
  }
}
