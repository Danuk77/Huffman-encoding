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

    pub fn left_child(&self) -> &Box<HuffmanTreeNode>{
        &self.left_child
    }

    pub fn right_child(&self) -> &Box<HuffmanTreeNode>{
        &self.right_child
    }
}

impl StoresFrequency for HuffmanTreeInnerNode {
    fn get_frequency(&self) -> u32 {
        return self.frequency;
    }
}
