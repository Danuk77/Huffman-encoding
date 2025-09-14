use crate::heap::MinHeap;

#[derive(Debug)]
pub struct MinPriorityQueue<T: Ord> {
    items: Vec<Box<T>>,
}

impl<T: Ord> MinHeap<T> for MinPriorityQueue<T> {
    fn insert(&mut self, item_to_insert: Box<T>) {
        self.items.push(item_to_insert);
        if self.items.len() == 1 {
            return;
        }
        self._bubble_up();
    }

    fn _bubble_up(&mut self) -> () {
        let mut _index_of_item_to_bubble_up = self.items.len() - 1;

        while _index_of_item_to_bubble_up > 0 {
            let _index_of_parent = (_index_of_item_to_bubble_up - 1) / 2;
            if self.items[_index_of_parent] <= self.items[_index_of_item_to_bubble_up] {
                return;
            }
            self.items
                .swap(_index_of_parent, _index_of_item_to_bubble_up);
            _index_of_item_to_bubble_up = _index_of_parent;
        }
    }

    fn get_min_element(&mut self) -> Option<Box<T>> {
        if self.items.len() == 0 {
            return None;
        }

        self._swap_root_with_last_element();
        let min_item = self.items.pop();
        self._bubble_down();

        min_item
    }

    fn _bubble_down(&mut self) -> () {
        let mut _index_of_item_to_bubble_down: usize = 0;

        loop {
            let mut _index_of_smallest_item = _index_of_item_to_bubble_down;
            let _left_child_index =
                self._get_left_child_index_if_exist(_index_of_item_to_bubble_down);
            let _right_child = self._get_right_child_index_if_exist(_index_of_item_to_bubble_down);

            // TODO: Change this to let Some
            self._update_index_of_smalles_item_if_child_is_smaller(
                &mut _index_of_smallest_item,
                _left_child_index,
            );
            self._update_index_of_smalles_item_if_child_is_smaller(
                &mut _index_of_smallest_item,
                _left_child_index,
            );

            if _index_of_smallest_item == _index_of_item_to_bubble_down {
                return;
            }

            self.items
                .swap(_index_of_smallest_item, _index_of_item_to_bubble_down);
            _index_of_item_to_bubble_down = _index_of_smallest_item;
        }
    }
}

impl<T: Ord> MinPriorityQueue<T> {
    pub fn new() -> MinPriorityQueue<T> {
        MinPriorityQueue { items: Vec::new() }
    }

    pub fn len(&self) -> usize{
        self.items.len()
    }

    pub fn is_empty(&self) -> bool{
        self.items.is_empty()
    }

    pub fn pop(&mut self) -> Option<Box<T>>{
        self.items.pop()
    }

    fn _swap_root_with_last_element(&mut self) -> () {
        let _index_of_last_item_in_queue = self.items.len() - 1;
        self.items.swap(0, _index_of_last_item_in_queue);
    }

    fn _get_left_child_index_if_exist(&self, index: usize) -> Option<usize> {
        let _index_of_left_child = (2 * index) + 1;

        if _index_of_left_child >= self.items.len() {
            return None;
        }

        Some(_index_of_left_child)
    }

    fn _get_right_child_index_if_exist(&self, index: usize) -> Option<usize> {
        let _index_of_right_child = (2 * index) + 2;

        if _index_of_right_child >= self.items.len() {
            return None;
        }

        Some(_index_of_right_child)
    }

    fn _update_index_of_smalles_item_if_child_is_smaller(
        &self,
        _current_smallest_item_index: &mut usize,
        _child_index: Option<usize>,
    ) -> () {
        match _child_index {
            None => {}
            Some(index) => {
                if self.items[index] < self.items[*_current_smallest_item_index] {
                    *_current_smallest_item_index = index;
                }
            }
        }
    }
}
