use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::SystemTime;

pub struct Config {
    filename: String
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("cannot get filename"),
        };

        Ok(Config { filename })
    }
}

pub fn run(config: Config) {
    let start_time = SystemTime::now();
    println!("filename: {}", config.filename);

    let mut reader = BufReader::new(File::open(config.filename)
        .expect("Cannot open file"));
    let mut counter = 0;
    let mut names: Vec<String> = Vec::new();
    let mut months: HashMap<String, i32> = HashMap::new();
    let mut most_names: HashMap<String, i32> = HashMap::new();

    let mut s = String::new();
    loop {
        s.clear();
        let res = reader.read_line(&mut s);
        if res.is_err() || res.unwrap() == 0 {
            break;
        } else {
            counter += 1;
            let line = s.clone();
            let splits: Vec<&str> = line.splitn(9, "|").collect();

            let date = splits[4];
            let name = splits[7];

            let month = date[..6].to_string();
            *months.entry(month).or_insert(0) += 1;

            if counter == 433 || counter == 43244 {
                names.push(name.to_string());
            }

            if name.contains(", ") {
                let split_name: Vec<&str> = name.splitn(2, ",").collect();
                *most_names.entry(split_name[0].to_string()).or_insert(0) += 1;
            }
        }
    }
    let elapsed_read_lines = start_time.elapsed().expect("error time lines");

    let mut max_val = 0;
    let mut max_name = "";
    for (k, v) in most_names.iter() {
        if *v > max_val {
            max_val = *v;
            max_name = k;
        }
    }
    let elapsed_max = start_time.elapsed().expect("error time max");

    println!("task 1:: {:?}", counter);
    println!("task 2:: {:?}", names);
    println!("task 3:: {:?}", months);
    println!("task 4:: {}, {}", max_name, max_val);

    // time
    println!("lines tooks:: {}ms", elapsed_read_lines.as_millis());
    println!("max tooks:: {}ms", elapsed_max.as_millis());
}