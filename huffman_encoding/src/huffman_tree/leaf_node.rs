use crate::huffman_tree::StoresFrequency;

pub struct HuffmanTreeLeafNode {
    frequency: u32,
    character: String,
}

impl StoresFrequency for HuffmanTreeLeafNode {
    fn get_frequency(&self) -> u32 {
        self.frequency
    }
}

impl HuffmanTreeLeafNode {
    pub fn new(frequency: u32, character: String) -> HuffmanTreeLeafNode {
        HuffmanTreeLeafNode {
            frequency: frequency,
            character: character,
        }
    }

    pub fn get_symbol(&self) -> String{
        self.character.clone()
    }
}
