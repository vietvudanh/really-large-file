use std::env;

fn main() {
    let mut args = env::args();
    args.next();
    let file_name = args.next().expect("cannot get filename");
    large_file::run(file_name);
}
