pub mod compile;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 3 {
        println!("Usage: {} <pathname> <output_filename>", args[0]);
        return;
    }

    let pathname = &args[1];
    let output_filename = &args[2];

    std::fs::remove_file(output_filename).unwrap_or(());

    compile::compile(pathname, output_filename);
}
