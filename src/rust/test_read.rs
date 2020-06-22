use std::io::BufReader;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let f : File = File::open("log.txt")?;
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    let len = reader.read_line(&mut line)?;
    println!("First line is {:?} bytes long", len);
    Ok(())
}