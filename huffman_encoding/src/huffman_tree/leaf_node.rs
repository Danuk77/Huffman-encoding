use crate::huffman_tree::HuffmanTreeNode;

pub struct HuffmanTreeLeafNode {
    frequency: u32,
    character: String,
}

impl HuffmanTreeNode for HuffmanTreeLeafNode {
    fn get_frequency(&self) -> u32 {
        return self.frequency;
    }
}

impl HuffmanTreeLeafNode {
    pub fn new(frequency: u32, character: String) -> HuffmanTreeLeafNode {
        HuffmanTreeLeafNode {
            frequency: frequency,
            character: character,
        }
    }
}
