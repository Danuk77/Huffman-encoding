use crate::heap::{MinHeap, MinPriorityQueue};
use crate::huffman_tree::{HuffmanTreeInnerNode, HuffmanTreeLeafNode, HuffmanTreeNode};

pub fn populate_huffman_queue_with_symbols(
    queue: &mut MinPriorityQueue<HuffmanTreeNode>,
    symbols_to_add: Vec<(u32, String)>,
) {
    for symbol in symbols_to_add {
        let _new_leaf = Box::new(HuffmanTreeNode::LeafNode(HuffmanTreeLeafNode::new(
            symbol.0, symbol.1,
        )));
        queue.insert(_new_leaf);
    }
}

pub fn generate_huffman_tree(
    queue: &mut MinPriorityQueue<HuffmanTreeNode>,
) -> Option<Box<HuffmanTreeNode>> {
    if queue.is_empty() {
        return None;
    }

    loop {
        if queue.len() == 1 {
            return Some(queue.pop().unwrap());
        }

        _run_huffman_iteration(queue);
    }
}

fn _run_huffman_iteration(queue: &mut MinPriorityQueue<HuffmanTreeNode>) {
    let _least_frequent_symbol = queue.pop().expect("Should not reach here 1");
    let _second_least_frequent_symbol = queue.pop().expect("Should not reach here 2");

    let _new_inner_node = Box::new(HuffmanTreeNode::InnerNode(HuffmanTreeInnerNode::new(
        _least_frequent_symbol,
        _second_least_frequent_symbol,
    )));

    queue.insert(_new_inner_node);
}
