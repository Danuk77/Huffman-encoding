// TODO: Implementation tasks for Huffman encoding
//  1. Implement structures used
//  2. Implement a minimum heap
//  3. Implement the huffman encoding algorithm

pub trait HuffmanTreeNode {
    fn get_frequency(&self) -> u32;
}

struct HuffmanTreeInnerNode<'a, T: HuffmanTreeNode> {
    frequency: u32,
    left_child: &'a T,
    right_child: &'a T,
}

impl<'a, T: HuffmanTreeNode> HuffmanTreeInnerNode<'a, T> {
    fn new(left_node: &'a T, right_node: &'a T) -> HuffmanTreeInnerNode<'a, T> {
        let frequency = left_node.get_frequency() + right_node.get_frequency();
        HuffmanTreeInnerNode {
            frequency: frequency,
            left_child: left_node,
            right_child: right_node,
        }
    }
}

impl<'a, T: HuffmanTreeNode> HuffmanTreeNode for HuffmanTreeInnerNode<'a, T> {
    fn get_frequency(&self) -> u32 {
        return self.frequency;
    }
}

struct HuffmanTreeLeafNode {
    frequency: u32,
    character: String,
}

impl HuffmanTreeNode for HuffmanTreeLeafNode {
    fn get_frequency(&self) -> u32 {
        return self.frequency;
    }
}

impl HuffmanTreeLeafNode {
    fn new(frequency: u32, character: String) -> HuffmanTreeLeafNode {
        HuffmanTreeLeafNode {
            frequency: frequency,
            character: character,
        }
    }
}

fn main() {
    let s = String::from("hello"); // s comes into scope
}
