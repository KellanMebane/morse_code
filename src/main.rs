mod bst;
use bst::*;

fn main() {
    let mut bst = BST::new();
    bst.insert(Pair::new('c', String::from("string")));
    bst.insert(Pair::new('b', String::from("string")));
}
