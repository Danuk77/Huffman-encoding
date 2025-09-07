pub trait HuffmanTreeNode {
    fn get_frequency(&self) -> u32;
}

pub mod inner_node;
pub mod leaf_node;

pub use inner_node::HuffmanTreeInnerNode;
pub use leaf_node::HuffmanTreeLeafNode;
