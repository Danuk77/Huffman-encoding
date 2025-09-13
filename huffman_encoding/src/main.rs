// TODO: Implementation tasks for Huffman encoding
//  1. Implement structures used
//  2. Implement a minimum heap
//  3. Implement the huffman encoding algorithm

mod huffman_tree;
mod min_heap;
use huffman_tree::{HuffmanTreeNode, HuffmanTreeLeafNode, HuffmanTreeInnerNode};
//use min_heap::MinPriorityQueue;

fn main() {
    // Create the leaf nodes
    let c = Box::new(HuffmanTreeLeafNode::new(12, String::from("c")));
    let d = Box::new(HuffmanTreeLeafNode::new(13, String::from("d")));
    let a = Box::new(HuffmanTreeLeafNode::new(5, String::from("a")));
    let b = Box::new(HuffmanTreeLeafNode::new(9, String::from("b")));
    let e = Box::new(HuffmanTreeLeafNode::new(16, String::from("e")));

    // Create the inner nodes
    let cd = Box::new(HuffmanTreeInnerNode::new(c, d));
    let ab = Box::new(HuffmanTreeInnerNode::new(a, b));
    let abe = Box::new(HuffmanTreeInnerNode::new(ab, e));
}
