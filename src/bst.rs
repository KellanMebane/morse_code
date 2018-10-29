// array based BST (SEQUENTIAL)

#[derive(Clone)]
pub struct Pair(char, String);

#[allow(dead_code)]
impl Pair {
    pub fn new(c: char, s: String) -> Pair {
        Pair { 0: c, 1: s }
    }
}

pub struct BST {
    size: usize,
    vec: Vec<Option<Pair>>, // vector of optional indeces containing
                            // either none or some pair of character and string
}

// parent(index) = [(index - 1) / 2] if r != 0
// left(index) = 2(index) + 1 if 2(index) + 1 <= size
// right(index) = 2(index) + 2 if 2(index + 2) <= size
#[allow(dead_code)]
impl BST {
    pub fn new() -> BST {
        BST {
            size: 100,
            vec: vec![None; 100],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn insert(&mut self, item: Pair) -> bool {
        if self.size == 0 {
            return false;
        }

        self.internal_insert(0, item)
    }

    // basically checks if the index is used
    // returns the char at the index or null char
    fn get_char(&self, index: usize) -> char {
        match self.vec[index].as_ref().as_ref() {
            Some(value) => value.0,
            None => 0 as char, // null I think lmao
        }
    }

    fn internal_insert(&mut self, index: usize, item: Pair) -> bool {
        let key = self.get_char(index);

        // this means we have an unused index
        // insert here
        if key == 0 as char {
            self.vec[index] = Some(item);
            return true;
        }

        if item.0 == key {
            return false;
        } else if item.0 > key {
            let mut new_index = 0;
            if (2 * index) + 2 <= self.size {
                new_index = (2 * index) + 2;
            }
            return self.internal_insert(new_index, item);
        } else {
            let mut new_index = 0;
            if (2 * index) + 1 <= self.size {
                new_index = (2 * index) + 1;
            }
            return self.internal_insert(new_index, item);
        }
    }

    pub fn inorder(&self) {
        self.internal_inorder(0);
    }

    fn internal_inorder(&self, index: usize) {
        let key = self.get_char(index);

        if key == 0 as char {
            return;
        }

        self.internal_inorder((2 * index) + 1);
        println!("{}", key);
        self.internal_inorder((2 * index) + 2);
    }
}
