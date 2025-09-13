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
    let c = Box::new(HuffmanTreeNode::LeafNode(HuffmanTreeLeafNode::new(12, String::from("c"))));
    let d = Box::new(HuffmanTreeNode::LeafNode(HuffmanTreeLeafNode::new(13, String::from("d"))));
    let a = Box::new(HuffmanTreeNode::LeafNode(HuffmanTreeLeafNode::new(5, String::from("a"))));
    let b = Box::new(HuffmanTreeNode::LeafNode(HuffmanTreeLeafNode::new(9, String::from("b"))));
    let e = Box::new(HuffmanTreeNode::LeafNode(HuffmanTreeLeafNode::new(16, String::from("e"))));
    let f = Box::new(HuffmanTreeNode::LeafNode(HuffmanTreeLeafNode:: new(45, String::from("f"))));

    // Create the inner nodes
    let cd = Box::new(HuffmanTreeNode::InnerNode(HuffmanTreeInnerNode::new(c, d)));
    let ab = Box::new(HuffmanTreeNode::InnerNode(HuffmanTreeInnerNode::new(a, b)));
    let abe = Box::new(HuffmanTreeNode::InnerNode(HuffmanTreeInnerNode::new(ab, e)));
    let cdabe = Box::new(HuffmanTreeNode::InnerNode(HuffmanTreeInnerNode::new(cd, abe)));
    let fcdabe = Box::new(HuffmanTreeNode::InnerNode(HuffmanTreeInnerNode::new(f, cdabe)));
}
