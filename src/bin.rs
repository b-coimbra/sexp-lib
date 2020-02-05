use std::env;
use std::fs::File;
use std::io::Read;

mod lexer;
mod parser;
mod ext;

fn is_bin(path: &String) -> bool {
    if let Some(bin_name) = env::args().nth(0) {
        return path == bin_name.as_str()
    }

    return false
}

fn read_file(filename: String) -> () {
    if is_bin(&filename) { return; }

    let mut content = String::new();

    let mut file = File::open(filename)
        .expect("Couldn't open file!");

    file.read_to_string(&mut content)
        .expect("Couldn't read file!");

    lexer::tokenize(content);
}

fn main() {
    env::args().for_each(read_file)
}
