use crate::huffman_tree::HuffmanTreeNode;

pub struct HuffmanTreeInnerNode<'a, T: HuffmanTreeNode> {
    frequency: u32,
    left_child: &'a T,
    right_child: &'a T,
}

impl<'a, T: HuffmanTreeNode> HuffmanTreeInnerNode<'a, T> {
    pub fn new(left_node: &'a T, right_node: &'a T) -> HuffmanTreeInnerNode<'a, T> {
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
