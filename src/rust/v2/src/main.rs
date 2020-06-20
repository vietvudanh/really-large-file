use std::env;
use std::process;
use large_file::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("problem parsig args: {}", err);
        process::exit(1);
    });

    large_file::run(config);
}
