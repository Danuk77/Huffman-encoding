use std::collections::HashMap;

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
    let _least_frequent_symbol = queue.get_min_element().expect("Should not reach here 1");
    let _second_least_frequent_symbol = queue.get_min_element().expect("Should not reach here 2");

    let _new_inner_node = Box::new(HuffmanTreeNode::InnerNode(HuffmanTreeInnerNode::new(
        _least_frequent_symbol,
        _second_least_frequent_symbol,
    )));

    queue.insert(_new_inner_node);
}

pub fn generate_prefix_codes(tree_root: &HuffmanTreeNode) -> HashMap<String, String> {
    let mut prefix_codes = HashMap::<String, String>::new();
    _traverse_tree_and_generate_prefix_codes(tree_root, String::new(), &mut prefix_codes);
    prefix_codes
}

fn _traverse_tree_and_generate_prefix_codes(
    huffman_node: &HuffmanTreeNode,
    accumulated_prefix: String,
    prefix_codes: &mut HashMap<String, String>,
) {
    match huffman_node {
        HuffmanTreeNode::LeafNode(leaf_node) => {
            prefix_codes.insert(leaf_node.get_symbol(), accumulated_prefix.clone());
        }
        HuffmanTreeNode::InnerNode(inner_node) => {
            _traverse_tree_and_generate_prefix_codes(
                inner_node.left_child(),
                format!("{}0", accumulated_prefix),
                prefix_codes,
            );
            _traverse_tree_and_generate_prefix_codes(
                inner_node.right_child(),
                format!("{}0", accumulated_prefix),
                prefix_codes,
            );
        }
    }
}
