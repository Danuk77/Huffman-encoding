pub mod inner_node;
pub mod leaf_node;

pub use inner_node::HuffmanTreeInnerNode;
pub use leaf_node::HuffmanTreeLeafNode;

pub trait StoresFrequency {
    fn get_frequency(&self) -> u32;
}

pub enum HuffmanTreeNode {
    InnerNode(HuffmanTreeInnerNode),
    LeafNode(HuffmanTreeLeafNode)
}

impl StoresFrequency for HuffmanTreeNode{
    fn get_frequency(&self) -> u32 {
        match self {
            HuffmanTreeNode::InnerNode(inner_node) => inner_node.get_frequency(),
            HuffmanTreeNode::LeafNode(leaf_node) => leaf_node.get_frequency()
        }
    }
}
