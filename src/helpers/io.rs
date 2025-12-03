use std::fs::File;
use std::io::prelude::*;

pub fn get_input(path: &str) -> Vec<String> {
    // TODO: tratar corretamente (remover expect)
    let mut file = File::open(path).expect("file should exist");

    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);

    contents.split('\n').map(|line| line.to_string()).collect()
}
