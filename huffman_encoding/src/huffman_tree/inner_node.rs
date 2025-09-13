use crate::huffman_tree::HuffmanTreeNode;

pub struct HuffmanTreeInnerNode<T: HuffmanTreeNode> {
    frequency: u32,
    left_child: Box<T>,
    right_child: Box<T>,
}

impl<T: HuffmanTreeNode> HuffmanTreeInnerNode<T> {
    pub fn new(left_node: Box<T>, right_node: Box<T>) -> HuffmanTreeInnerNode<T> {
        let frequency = left_node.get_frequency() + right_node.get_frequency();
        HuffmanTreeInnerNode {
            frequency: frequency,
            left_child: left_node,
            right_child: right_node,
        }
    }
}

impl<T: HuffmanTreeNode> HuffmanTreeNode for HuffmanTreeInnerNode<T> {
    fn get_frequency(&self) -> u32 {
        return self.frequency;
    }
}
