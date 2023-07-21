use std::fs;
use std::io::Write;

pub mod compile;

fn create_file(filename:&str) {
    let mut file = fs::File::create(filename).unwrap();

    let size = 40 << 20; // 40MB
    let buffer = vec![0; size];

    file.write_all(&buffer).unwrap();
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 3 {
        println!("Usage: {} <pathname> <output_filename>", args[0]);
        return;
    }

    let pathname = &args[1];
    let output_filename = &args[2];

    fs::remove_file(output_filename).unwrap_or(());
    create_file(output_filename);

    compile::compile(pathname, output_filename);
}
