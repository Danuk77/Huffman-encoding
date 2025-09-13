use crate::huffman_tree::{HuffmanTreeNode, StoresFrequency};

pub struct HuffmanTreeInnerNode {
    frequency: u32,
    left_child: Box<HuffmanTreeNode>,
    right_child: Box<HuffmanTreeNode>,
}

impl HuffmanTreeInnerNode {
    pub fn new(left_node: Box<HuffmanTreeNode>, right_node: Box<HuffmanTreeNode>) -> HuffmanTreeInnerNode {
        let frequency = left_node.get_frequency() + right_node.get_frequency();
        HuffmanTreeInnerNode {
            frequency: frequency,
            left_child: left_node,
            right_child: right_node,
        }
    }
}

impl StoresFrequency for HuffmanTreeInnerNode {
    fn get_frequency(&self) -> u32 {
        return self.frequency;
    }
}
