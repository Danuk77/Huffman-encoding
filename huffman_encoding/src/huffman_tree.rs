use std::cmp::Ordering;

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

impl Ord for HuffmanTreeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_frequency().cmp(&other.get_frequency())
    }
}

impl PartialOrd for HuffmanTreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HuffmanTreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.get_frequency() == other.get_frequency()
    }
}

impl Eq for HuffmanTreeNode {}
