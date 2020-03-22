use std::env;
use std::path;

mod analyzer;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cartridge_path = path::Path::new(&args[1]);
    let analyzer = analyzer::Analyzer::from_path(cartridge_path).unwrap();
    let disass = analyzer.disassemble();
    println!("{:#?}", disass);
}
