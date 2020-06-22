use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::SystemTime;

const BATCH_SIZE: usize = 10000;

pub fn run(filename: String) {
    let start_time = SystemTime::now();
    println!("filename: {}", filename);

    let file = File::open(filename)
        .expect("Cannot open file");
    let mut reader = BufReader::new(file);
    let mut counter = 0;

    let mut s = String::new();
    let mut lines: Vec<String> = Vec::new();
    let mut handles: Vec<thread::JoinHandle<_>> = Vec::new();
    loop {
        s.clear();
        let res = reader.read_line(&mut s);
        let mut eof = false;
        if res.is_err() || res.unwrap() == 0 {
            eof = true;
        }
        counter += 1;

        if lines.len() == BATCH_SIZE || eof {
            // new thread
            let handle = thread::spawn(move || {
                println!("thread len {}", lines.len());
            });
            lines = Vec::new();
            handles.push(handle);

            if eof {
                break;
            }
        }
    }
    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed_read_lines = start_time.elapsed()
        .expect("error time lines");
    println!("task 1:: {:?}", counter);

    // time
    println!("lines tooks:: {}ms", elapsed_read_lines.as_millis());
}