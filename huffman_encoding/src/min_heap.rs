pub trait MinHeap<T: Ord> {
    fn insert(&mut self, item_to_insert: Box<T>) -> ();
    fn bubble_up(&mut self) -> ();
    //fn bubble_down(&mu)
    fn peek(&mut self) -> Option<&Box<T>>;
}

pub struct MinPriorityQueue<T: Ord> {
    items: Vec<Box<T>>,
}

impl<T: Ord> MinHeap<T> for MinPriorityQueue<T> {
    fn insert(&mut self, item_to_insert: Box<T>) {
        self.items.push(item_to_insert);
        if self.items.len() == 1 {
            return;
        }
        self.bubble_up();
    }

    fn bubble_up(&mut self) -> () {
        let mut index_of_item_to_bubble_up = self.items.len() - 1;

        while index_of_item_to_bubble_up > 0 {
            let index_of_parent = (index_of_item_to_bubble_up - 1) / 2;
            if self.items[index_of_parent] <= self.items[index_of_item_to_bubble_up] {
                return;
            }
            self.items.swap(index_of_parent, index_of_item_to_bubble_up);
            index_of_item_to_bubble_up = index_of_parent;
        }
    }

    fn peek(&mut self) -> Option<&Box<T>> {
        self.items.first()
    }
}

impl<T: Ord> MinPriorityQueue<T> {
    pub fn new() -> MinPriorityQueue<T> {
        MinPriorityQueue { items: Vec::new() }
    }
}
