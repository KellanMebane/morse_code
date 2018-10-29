// array based BST (SEQUENTIAL)

#[derive(Clone)]
pub struct Pair(char, String);

pub struct BST {
    size: usize,
    vec: Vec<Option<Pair>>, // vector of optional indeces containing
                            // either none or some pair of character and string
}

// parent(index) = [(index - 1) / 2] if r != 0
// left(index) = 2(index) + 1 if 2(index) + 1 <= size
// right(index) = 2(index) + 2 if 2(index + 2) <= size

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

    fn internal_insert(&mut self, index: usize, item: Pair) -> bool {
        let mut pair;
        {
            let x = self.vec[index].as_ref();
            let y = x.as_ref();
            pair = match y {
                Some(value) => value.0,
                None => 0 as char, // null I think lmao
            }
        }

        if pair == 0 as char {
            
        }

        if item.0 == pair {
            return false;
        } else if item.0 > pair {
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
}
