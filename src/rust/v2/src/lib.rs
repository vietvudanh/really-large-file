use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::SystemTime;

// constants
const BATCH_SIZE: usize = 512 * 1024; // process in each thread

pub struct Config {
    filename: String
}

struct Entry {
    index: i64,
    first_name: String,
    name: String,
    month: String,
}

struct LineIndex {
    index: i64,
    line: String,
}
impl Clone for LineIndex {
    fn clone(&self) -> LineIndex {
        LineIndex {
            index: (*self).index,
            line: (*self).line.to_string(),
        }
    }
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

    let mut reader = BufReader::new(
        File::open(config.filename)
            .expect("Cannot open file"));

    let mut counter: i64 = 0;
    let mut_names: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let mut_months: Arc<Mutex<HashMap<String, i32>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut_fname_count: Arc<Mutex<HashMap<String, i32>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut_max_name: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    let mut_max_name_val: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

    let mut handles = vec![];
    let mut lines: Vec<LineIndex> = Vec::new();
    let mut s = String::new();
    loop {
        s.clear();
        let mut eof: bool = false;

        let res = reader.read_line(&mut s);
        if res.is_err() || res.unwrap() == 0 {
            eof = true;
        } else {
            counter += 1;
            lines.push(LineIndex { index: counter, line: s.clone() });
        }

        if lines.len() == BATCH_SIZE || eof {
            // process line
            let lines_clone = lines.clone();

            let mut_names = Arc::clone(&mut_names);
            let mut_months = Arc::clone(&mut_months);
            let mut_fname_count = Arc::clone(&mut_fname_count);
            let mut_max_name = Arc::clone(&mut_max_name);
            let mut_max_name_val = Arc::clone(&mut_max_name_val);

            let handle = thread::spawn(move || {
                let mut entries: Vec<Entry> = Vec::new();
                for thread_line in &lines_clone {
                    let splits: Vec<&str> = thread_line.line.splitn(9, "|").collect();

                    let date = splits[4];
                    let name = splits[7];
                    let mut first_name: String = String::new();

                    let month = date[..6].to_string();
                    if name.contains(", ") {
                        let split_name: Vec<&str> = name.splitn(2, ",").collect();
                        first_name = split_name[0].to_string();
                    }

                    entries.push(Entry {
                        index: thread_line.index,
                        first_name,
                        name: name.to_string(),
                        month,
                    });
                }

                let mut months = mut_months.lock().unwrap();
                let mut fname_count = mut_fname_count.lock().unwrap();
                let mut names = mut_names.lock().unwrap();
                let mut max_name = mut_max_name.lock().unwrap();
                let mut max_name_val = mut_max_name_val.lock().unwrap();
                for entry in entries {
                    if &*entry.first_name != "" {
                        let find_key = entry.first_name;
                        *fname_count.entry(find_key.clone()).or_insert(0) += 1;
                        let find_val: i32 = *fname_count.get(&find_key).expect("");
                        if find_val > *max_name_val {
                            *max_name_val = find_val;
                            *max_name = find_key;
                        }
                    }
                    if entry.index == 433 || entry.index == 43244 {
                        (*names).push(entry.name);
                    }
                    *months.entry(entry.month).or_insert(0) += 1;
                }
            });
            handles.push(handle);

            // re-init lines
            if !eof {
                lines = Vec::new();
            }
        }

        if eof {
            break;
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }
    // end process
    let elapsed_read_lines = start_time.elapsed().expect("error time lines");

    let months = mut_months.lock().unwrap();
    let names = mut_names.lock().unwrap();
    let max_name = mut_max_name.lock().unwrap();
    let max_name_val = mut_max_name_val.lock().unwrap();

    println!("task 1:: {:?}", counter);
    println!("task 2:: {:?}", *names);
    println!("task 3:: {:?}", *months);
    println!("task 4:: {}, {}", *max_name, *max_name_val);

    // time
    println!("lines took:: {}ms", elapsed_read_lines.as_millis());
}