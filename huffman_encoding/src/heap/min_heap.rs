pub trait MinHeap<T: Ord> {
    fn insert(&mut self, item_to_insert: Box<T>) -> ();
    fn get_min_element(&mut self) -> Option<Box<T>>;
    fn _bubble_up(&mut self) -> ();
    fn _bubble_down(&mut self) -> ();
}
