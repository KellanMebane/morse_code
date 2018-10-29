mod bst;
//use bst::*;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    //let mut bst = BST::new();
    let file = File::open("./src/res/morse.csv").expect("file not found");

    for file_line in BufReader::new(file).lines() {
        let line = file_line.unwrap();
        //ignore empty lines
        if line != "" {
            let data: Vec<&str> = line.split(",").collect();

            println!("{:?}", data);
        }
    }
}
