mod bst;
use bst::*;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let mut bst = BST::new();
    let file = File::open("./src/res/morse.txt").expect("file not found");
    let input = File::open("./src/res/message.txt").expect("file not found");

    for file_line in BufReader::new(file).lines() {
        let line = file_line.unwrap();
        //ignore empty lines
        if line != "" {
            let data: Vec<&str> = line.split(" ").collect();
            let temp: Vec<_> = data[0].to_string().chars().collect();
            bst.insert(Pair::new(temp[0], data[1].to_string()));
            //println!("{:?}", data);
        }
    }

    //bst.inorder();

    for file_line in BufReader::new(input).lines() {
        let line = file_line.unwrap().to_uppercase();
        let str_vec: Vec<_> = line.chars().collect();
        for letter in str_vec {
            if letter != ' ' {
                let morse = bst.search(letter);
                //println!("{}{}", morse, if letter == ' ' { " " } else { "   " });
                print!("{} ", morse);
            }
            else {
                print!("   ");
            }
        }

        println!("");
    }
}
