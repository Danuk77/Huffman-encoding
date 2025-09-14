mod heap;
mod huffman;
mod huffman_tree;

use heap::MinPriorityQueue;
use huffman_tree::HuffmanTreeNode;
use huffman::{populate_huffman_queue_with_symbols, generate_huffman_tree, generate_prefix_codes};

fn main() {
    let mut queue = MinPriorityQueue::<HuffmanTreeNode>::new();
    let symbols_to_add = vec![
        (12, "c".to_string()),
        (13, "d".to_string()),
        (5, "a".to_string()),
        (9, "b".to_string()),
        (16, "e".to_string()),
        (45, "f".to_string()),
    ];
    populate_huffman_queue_with_symbols(&mut queue, symbols_to_add);
    println!("{:?}", queue);
    let _huffman_tree = generate_huffman_tree(&mut queue).unwrap();
    let _prefix_codes = generate_prefix_codes(&_huffman_tree);

    println!("{:?}", _prefix_codes);
}
