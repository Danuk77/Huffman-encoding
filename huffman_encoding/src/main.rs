mod heap;
mod huffman;
mod huffman_tree;

use heap::MinPriorityQueue;
use huffman::{populate_huffman_queue_with_symbols, generate_huffman_tree};
use huffman_tree::HuffmanTreeNode;

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
    let _huffman_tree = generate_huffman_tree(&mut queue).unwrap();
    
    // Create the inner nodes
    //let cd = Box::new(HuffmanTreeNode::InnerNode(HuffmanTreeInnerNode::new(c, d)));
    //let ab = Box::new(HuffmanTreeNode::InnerNode(HuffmanTreeInnerNode::new(a, b)));
    //let abe = Box::new(HuffmanTreeNode::InnerNode(HuffmanTreeInnerNode::new(ab, e)));
    //let cdabe = Box::new(HuffmanTreeNode::InnerNode(HuffmanTreeInnerNode::new(cd, abe)));
    //let fcdabe = Box::new(HuffmanTreeNode::InnerNode(HuffmanTreeInnerNode::new(f, cdabe)));

}
