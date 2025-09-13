use crate::HuffmanTreeNode;

pub trait MinHeap<T: HuffmanTreeNode>{
    fn insert(&mut self, item_to_insert: T) -> ();
    fn peek(&mut self) -> Option<&T>;
}

pub struct MinPriorityQueue<T: HuffmanTreeNode> {
    items: Vec<T>
}

impl<T:HuffmanTreeNode> MinHeap<T> for MinPriorityQueue<T> {
    fn insert(&mut self, item_to_insert: T) -> () {
        self.items.push(item_to_insert);
        // TODO: 
        //  1. Append to the end of the vector
        //  2. Check upwards if the parent is less than the new added item
    }

    fn peek(&mut self) -> Option<&T>{
        self.items.first()
    }
}

impl<T:HuffmanTreeNode> MinPriorityQueue<T>{
    pub fn new() -> MinPriorityQueue<T>{
       MinPriorityQueue { items: Vec::new() } 
    }
}
