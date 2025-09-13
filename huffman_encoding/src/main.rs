// TODO: Implementation tasks for Huffman encoding
//  1. Implement structures used
//  2. Implement a minimum heap
//  3. Implement the huffman encoding algorithm

mod huffman_tree;
mod min_heap;
use huffman_tree::{HuffmanTreeNode, HuffmanTreeLeafNode, HuffmanTreeInnerNode};
use min_heap::MinPriorityQueue;

fn main() {
    let test_node = HuffmanTreeLeafNode::new(5, String::from("a"));
    let priority_queue = MinPriorityQueue<impl HuffmanTreeNode>::new();
}
